#![deny(clippy::unwrap_used, clippy::expect_used)]

use rand::Rng;
use scrypt::{Params, scrypt};
use sha2::{Digest, Sha256};
use sha3::Sha3_512;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{error, info};

// Current version of our file format
pub const VERSION: u8 = 1;

// This function generates random passwords with given length and count
pub fn gen_passwords(
    chars: usize,   // How long each passwords would be
    num: usize,     // How many passwords to make
    threads: usize, // How many threads to use
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Basic input check
    if chars == 0 || num == 0 || threads == 0 {
        return Err("chars, num, and threads must be greater than zero".into());
    }

    info!(
        "Starting password generation: {} passwords, length {}, threads {}",
        num, chars, threads
    );

    // ASCII characters we can use from space ' ' to tilde '~' everything
    let allowed: Vec<u8> = (32u8..=126u8).collect();

    // Shared list where threads will store generated passwords
    let passwords = Arc::new(Mutex::new(Vec::with_capacity(num)));

    // Figure out how many passwords each thread should make
    let per_thread = num / threads;
    let remainder = num % threads;

    let mut handles = Vec::new();
    for i in 0..threads {
        let allowed = allowed.clone();
        let passwords_clone = Arc::clone(&passwords);

        // Some threads might do 1 extra password if the count isn't even
        let count = per_thread + if i < remainder { 1 } else { 0 };
        let handle = thread::spawn(move || {
            info!("Thread {} generating {} passwords", i, count);
            let mut local = Vec::with_capacity(count);
            let mut rng = rand::thread_rng(); // Each thread gets its own random generator

            // Build a password by picking random characters
            for _ in 0..count {
                let pwd: String = (0..chars)
                    .map(|_| {
                        let idx = rng.gen_range(0..allowed.len());
                        allowed[idx] as char
                    })
                    .collect();
                local.push(pwd);
            }

            // Safely add our passwords to the shared list
            if let Ok(mut pwds) = passwords_clone.lock() {
                pwds.extend(local);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for h in handles {
        let _ = h.join();
    }

    // Gets our final password list back from the shared state
    match Arc::try_unwrap(passwords) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(result) => Ok(result),
            Err(e) => {
                error!("Mutex into_inner error: {e}");
                Err(format!("Mutex into_inner error: {e}").into())
            }
        },
        Err(_) => {
            error!("Arc had multiple strong references");
            Err("Arc had multiple strong references".into())
        }
    }
}

// This function takes passwords from a file, hashes them, and saves to another file
pub fn gen_hashes(
    in_file: &str,   // File with the passwords
    out_file: &str,  // File where we want to save hashes
    algorithm: &str, // Like "sha256" and etc..
    threads: usize,  // How many threads to use
) -> Result<(), Box<dyn std::error::Error>> {
    if threads == 0 {
        return Err("threads must be greater than zero".into());
    }

    // Read all passwords from the input file
    let file = File::open(in_file)?;
    let reader = BufReader::new(file);
    let passwords: Vec<String> = reader.lines().map_while(Result::ok).collect();

    if passwords.is_empty() {
        return Err("No passwords found in input file".into());
    }

    info!(
        "Loaded {} passwords from file '{}'",
        passwords.len(),
        in_file
    );
    info!(
        "Hashing passwords using '{}' algorithm across {} threads",
        algorithm, threads
    );

    // Check all passwords are same length
    let pwd_len = passwords[0].len();
    if !passwords.iter().all(|pwd| pwd.len() == pwd_len) {
        return Err("Not all passwords have the same length".into());
    }

    // Shared list for threads to store hashes
    let hashes = Arc::new(Mutex::new(Vec::with_capacity(passwords.len())));

    // Spliting the work between threads
    let chunk_size = passwords.len().div_ceil(threads);
    let mut handles = Vec::new();

    for chunk in passwords.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let algorithm = algorithm.to_string();
        let hashes_arc = Arc::clone(&hashes);
        let handle = thread::spawn(move || {
            info!("Thread hashing {} passwords", chunk.len());
            let mut local_hashes = Vec::with_capacity(chunk.len());
            for pwd in chunk {
                if let Ok(hash_bytes) = compute_hash(&pwd, &algorithm) {
                    local_hashes.push(hash_bytes);
                }
            }

            // Safely add hashes to our shared list
            if let Ok(mut h) = hashes_arc.lock() {
                h.extend(local_hashes);
            }
        });
        handles.push(handle);
    }

    // Wait for threads to finish
    for h in handles {
        let _ = h.join();
    }

    // Get all hashes back from shared state
    let hashes = match Arc::try_unwrap(hashes) {
        Ok(mutex) => match mutex.into_inner() {
            Ok(h) => h,
            Err(e) => {
                error!("Mutex into_inner error: {e}");
                return Err(format!("Mutex into_inner error: {e}").into());
            }
        },
        Err(_) => {
            error!("Arc had multiple strong references");
            return Err("Arc had multiple strong references".into());
        }
    };

    // Prepare output file format
    let mut output = Vec::new();
    output.push(VERSION);
    let algo_lower = algorithm.to_lowercase();
    output.push(algo_lower.len() as u8);
    output.extend_from_slice(algo_lower.as_bytes());
    output.push(pwd_len as u8);

    // Add all hashes to the output
    for hash in hashes {
        output.extend_from_slice(&hash);
    }

    // Write everything to the output file
    let mut out = File::create(out_file)?;
    out.write_all(&output)?;
    Ok(())
}

// This function shows hash file contents in human-readable format
pub fn dump_hashes(in_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read entire file into memory
    let data = std::fs::read(in_file)?;

    // Basic file format validation
    if data.len() < 3 {
        return Err("Invalid file format: file too short".into());
    }

    let version = data[0]; // First byte is version
    let algo_len = data[1] as usize; // Second byte is algo name length

    // Check we have enough data for the header
    if data.len() < 2 + algo_len + 1 {
        return Err("Invalid file format: insufficient header data".into());
    }

    // Get algorithm name
    let algo_bytes = &data[2..2 + algo_len];
    let algorithm = String::from_utf8(algo_bytes.to_vec())
        .map_err(|e| format!("Invalid UTF-8 in algorithm string: {e}"))?;

    let pwd_len = data[2 + algo_len]; // Password length byte

    // Figuring out how long each hash should be
    let hash_len = match algorithm.as_str() {
        "md5" => 16,
        "sha256" => 32,
        "sha3_512" => 64,
        "scrypt" => 32,
        _ => return Err(format!("Unsupported algorithm in file: {algorithm}").into()),
    };

    // Print file information
    println!("VERSION: {}", version);
    println!("ALGORITHM: {}", algorithm);
    println!("PASSWORD LENGTH: {}", pwd_len);

    // Extract and print all hashes
    let mut pos = 2 + algo_len + 1;
    let mut hash_lines = Vec::new();
    while pos + hash_len <= data.len() {
        let hash_bytes = &data[pos..pos + hash_len];
        let hash_str = if algorithm == "scrypt" {
            // Scrypt outputs raw bytes that might not be UTF-8
            String::from_utf8_lossy(hash_bytes).to_string()
        } else {
            // All other algorithms get shown as hex strings
            hex::encode(hash_bytes)
        };
        hash_lines.push(hash_str);
        pos += hash_len;
    }

    // Warn if file has extra unused bytes
    if pos != data.len() {
        tracing::warn!(
            "Extra {} byte(s) at the end of the file were ignored.",
            data.len() - pos
        );
    }

    // Print all hashes
    for line in hash_lines {
        println!("{}", line);
    }

    Ok(())
}

// This is a helper function that actually computes a single hash
fn compute_hash(password: &str, algorithm: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match algorithm.to_lowercase().as_str() {
        "md5" => Ok(md5::compute(password).0.to_vec()), // MD5
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            Ok(hasher.finalize().to_vec()) // SHA-256
        }
        "sha3_512" => {
            let mut hasher = Sha3_512::new();
            hasher.update(password.as_bytes());
            Ok(hasher.finalize().to_vec()) // SHA3-512
        }
        "scrypt" => {
            let params = Params::new(14, 8, 1)?; // CPU/memory cost settings
            let mut output = [0u8; 32]; // Output buffer
            scrypt(
                password.as_bytes(), // Password as bytes
                password.as_bytes(),
                &params,     //Cost parameters
                &mut output, // Where to put the results
            )?;
            Ok(output.to_vec())
        }
        _ => Err("Unsupported algorithm".into()),
    }
}
