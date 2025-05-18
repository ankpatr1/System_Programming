#![deny(clippy::unwrap_used, clippy::expect_used)]

use rand::Rng;
use scrypt::{Params, scrypt};
use sha2::{Digest, Sha256};
use sha3::Sha3_512;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use stretto::Cache;
use tracing::info;

// Current version of our file format
pub const VERSION: u8 = 1;

// This is a helper function that actually computes a single hash
fn compute_hash(password: &str, algorithm: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match algorithm.to_lowercase().as_str() {
        "md5" => Ok(md5::compute(password).0.to_vec()), // MD5
        "sha256" => Ok(Sha256::digest(password.as_bytes()).to_vec()), // SHA-256
        "sha3_512" => Ok(Sha3_512::digest(password.as_bytes()).to_vec()), // SHA3-512
        "scrypt" => {
            let params = Params::new(14, 8, 1)?; // CPU/memory cost settings
            let mut output = [0u8; 32];
            scrypt(
                password.as_bytes(), // Password as bytes
                password.as_bytes(),
                &params,     //Cost parameters
                &mut output, // Where to put the results
            )?;
            Ok(output.to_vec())
        }
        _ => Err(format!("Unsupported algorithm: {}", algorithm).into()),
    }
}

fn reduce_hash(hash: &[u8], length: usize, allowed: &[u8]) -> String {
    let mut pwd = String::with_capacity(length);
    let charset_len = allowed.len();
    for i in 0..length {
        let idx = (hash[i % hash.len()] as usize) % charset_len;
        pwd.push(allowed[idx] as char);
    }
    pwd
}

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
        let passwords = Arc::clone(&passwords);

        // Some threads might do 1 extra password if the count isn't even
        let count = per_thread + if i < remainder { 1 } else { 0 };

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng(); // Each thread gets its own random generator
            let mut local = Vec::with_capacity(count);

            // Build a password by picking random characters
            for _ in 0..count {
                let pwd: String = (0..chars)
                    .map(|_| allowed[rng.gen_range(0..allowed.len())] as char)
                    .collect();
                local.push(pwd);
            }

            // Safely add our passwords to the shared list
            if let Ok(mut pwds) = passwords.lock() {
                pwds.extend(local);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    // Gets our final password list back from the shared state
    Arc::try_unwrap(passwords)
        .map_err(|_| "Arc had multiple strong references".to_string())?
        .into_inner()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// This function takes passwords from a file, hashes them, and saves to another file
pub fn gen_hashes(
    in_file: &str,   // File with the passwords
    out_file: &str,  // File where we want to save hashes
    algorithm: &str, // Like "sha256" and etc..
    threads: usize,  // How many threads to use
) -> Result<(), Box<dyn std::error::Error>> {
    // Read all passwords from the input file
    let file = File::open(in_file)?;
    let reader = BufReader::new(file);
    let passwords: Vec<String> = reader.lines().map_while(Result::ok).collect();

    if passwords.is_empty() {
        return Err("No passwords found".into());
    }

    // Check all passwords are same length
    let pwd_len = passwords[0].len();
    if !passwords.iter().all(|pwd| pwd.len() == pwd_len) {
        return Err("Passwords must all have the same length".into());
    }

    // Shared list for threads to store hashes
    let hashes = Arc::new(Mutex::new(Vec::new()));

    // Spliting the work between threads
    let chunk_size = passwords.len().div_ceil(threads);

    let mut handles = Vec::new();
    for chunk in passwords.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let algo = algorithm.to_string();
        let hashes = Arc::clone(&hashes);

        let handle = thread::spawn(move || {
            let mut local = Vec::new();
            for pwd in chunk {
                // Safely add hashes to our shared list
                if let Ok(hash) = compute_hash(&pwd, &algo) {
                    local.push(hash);
                }
            }
            if let Ok(mut h) = hashes.lock() {
                h.extend(local);
            }
        });

        handles.push(handle);
    }

    // Wait for threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    // Get all hashes back from shared state
    let hashes = Arc::try_unwrap(hashes)
        .map_err(|_| "Arc had multiple strong references".to_string())?
        .into_inner()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Prepare output file format
    let mut output = Vec::new();
    output.push(VERSION);
    output.push(algorithm.len() as u8);
    output.extend_from_slice(algorithm.as_bytes());
    output.push(pwd_len as u8);

    // Add all hashes to the output
    for hash in hashes {
        output.extend_from_slice(&hash);
    }

    // Write everything to the output file
    let mut f = File::create(out_file)?;
    f.write_all(&output)?;

    Ok(())
}

// This function shows hash file contents in human-readable format
pub fn dump_hashes(in_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read entire file into memory
    let data = std::fs::read(in_file)?;

    let version = data[0]; // First byte is version
    let algo_len = data[1] as usize; // Second byte is algo name length
    let algo = String::from_utf8(data[2..2 + algo_len].to_vec())?;
    let pwd_len = data[2 + algo_len];

    // Print file information
    println!("VERSION: {version}");
    println!("ALGORITHM: {algo}");
    println!("PASSWORD LENGTH: {pwd_len}");

    let mut pos = 2 + algo_len + 1;

    // Figuring out how long each hash should be
    let hash_len = match algo.as_str() {
        "md5" => 16,
        "sha256" => 32,
        "sha3_512" => 64,
        "scrypt" => 32,
        _ => return Err("Unsupported algorithm".into()),
    };

    while pos + hash_len <= data.len() {
        println!("{}", hex::encode(&data[pos..pos + hash_len]));
        pos += hash_len;
    }

    Ok(())
}

// Function to generate a rainbow table from a list of passwords
pub fn gen_rainbow_table(
    in_file: &str,
    out_file: &str,
    algorithm: &str,
    num_links: usize,
    threads: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input file containing seed passwords
    let file = File::open(in_file)?;
    let reader = BufReader::new(file);
    let seeds: Vec<String> = reader.lines().map_while(Result::ok).collect();

    if seeds.is_empty() {
        return Err("No seeds found".into());
    }

    let pwd_len = seeds[0].len();
    // Check that all passwords have the same length as the first one
    if !seeds.iter().all(|pwd| pwd.len() == pwd_len) {
        return Err("Input file contains passwords of varying lengths".into());
    }

    // Define the allowed character set (ASCII printable characters)
    let allowed: Vec<u8> = (32u8..=126u8).collect();

    // Shared vector to store the start and end of each chain
    let chains = Arc::new(Mutex::new(Vec::new()));

    // Determine the number of seeds each thread should process
    let chunk_size = seeds.len().div_ceil(threads);

    let mut handles = Vec::new();
    for chunk in seeds.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let algo = algorithm.to_string();
        let chains = Arc::clone(&chains);
        let allowed = allowed.clone();

        // Spawn a new thread for each chunk
        let handle = thread::spawn(move || {
            let mut local = Vec::new();
            for mut pwd in chunk {
                let start = pwd.clone();
                // Apply hash and reduction functions num_links times
                for _ in 0..num_links {
                    if let Ok(hash) = compute_hash(&pwd, &algo) {
                        pwd = reduce_hash(&hash, pwd.len(), &allowed);
                    }
                }
                // Store the start and end of the chain
                local.push((start, pwd));
            }
            // Safely add our chains to the shared list
            if let Ok(mut c) = chains.lock() {
                c.extend(local);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    // Get all chains back from shared state
    let chains = Arc::try_unwrap(chains)
        .map_err(|_| "Arc had multiple strong references".to_string())?
        .into_inner()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Write the chains to the output file
    let mut f = File::create(out_file)?;
    f.write_all(b"rainbowtable")?;
    f.write_all(&[VERSION])?;
    f.write_all(&[algorithm.len() as u8])?;
    f.write_all(algorithm.as_bytes())?;
    f.write_all(&[pwd_len as u8])?;
    f.write_all(&(allowed.len() as u128).to_be_bytes())?;
    f.write_all(&(num_links as u128).to_be_bytes())?;
    f.write_all(&[32u8])?;

    // Write each chain's start and end passwords
    for (start, end) in chains {
        f.write_all(start.as_bytes())?;
        f.write_all(end.as_bytes())?;
    }

    Ok(())
}

// Function to dump the contents of a rainbow table
pub fn dump_rainbow_table(in_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the entire file into memory
    let data = std::fs::read(in_file)?;
    // Verify the magic header
    if &data[..12] != b"rainbowtable" {
        return Err("Invalid rainbow table format".into());
    }

    let mut pos = 12;
    let version = data[pos];
    pos += 1;
    let algo_len = data[pos] as usize;
    pos += 1;
    let algo = String::from_utf8(data[pos..pos + algo_len].to_vec())?;
    pos += algo_len;
    let pwd_len = data[pos];
    pos += 1;
    let charset_size = u128::from_be_bytes(data[pos..pos + 16].try_into()?);
    pos += 16;
    let num_links = u128::from_be_bytes(data[pos..pos + 16].try_into()?);
    pos += 16;
    let ascii_offset = data[pos];
    pos += 1;

    // Print table metadata
    println!("Hashassin Rainbow Table");
    println!("VERSION: {version}");
    println!("ALGORITHM: {algo}");
    println!("PASSWORD LENGTH: {pwd_len}");
    println!("KEY SIZE: {charset_size}");
    println!("NUM LINKS: {num_links}");
    println!("ASCII OFFSET: {ascii_offset}");

    // Iterate over each chain and print start and end passwords
    let chain_len = (pwd_len as usize) * 2;
    while pos + chain_len <= data.len() {
        println!(
            "{}\t{}",
            String::from_utf8_lossy(&data[pos..pos + pwd_len as usize]),
            String::from_utf8_lossy(&data[pos + pwd_len as usize..pos + chain_len])
        );
        pos += chain_len;
    }

    Ok(())
}

// Function to crack hashes using a rainbow table
pub fn crack(
    table_file: &str,
    hashes_file: &str,
    out_file: Option<&str>,
    _threads: usize,
    cache: Option<&Cache<String, String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use tracing::info;

    info!("Reading rainbow table and hashes files");
    let table_data = std::fs::read(table_file)?;
    let hashes_data = std::fs::read(hashes_file)?;

    if &table_data[..12] != b"rainbowtable" {
        return Err("Invalid rainbow table format".into());
    }

    let mut pos = 12;
    let _version = table_data[pos];
    pos += 1;
    let algo_len = table_data[pos] as usize;
    pos += 1;
    let algorithm = String::from_utf8(table_data[pos..pos + algo_len].to_vec())?;
    pos += algo_len;
    let pwd_len = table_data[pos] as usize;
    pos += 1;
    let _charset_size = u128::from_be_bytes(table_data[pos..pos + 16].try_into()?);
    pos += 16;
    let num_links = u128::from_be_bytes(table_data[pos..pos + 16].try_into()?) as usize;
    pos += 16;
    let _ascii_offset = table_data[pos];
    pos += 1;

    info!(
        "Rainbow table: {} algorithm, {} password length, {} chain links",
        algorithm, pwd_len, num_links
    );

    let mut chains = HashMap::new();
    let chain_len = pwd_len * 2;
    while pos + chain_len <= table_data.len() {
        let start_pwd = String::from_utf8_lossy(&table_data[pos..pos + pwd_len]).to_string();
        let end_pwd =
            String::from_utf8_lossy(&table_data[pos + pwd_len..pos + chain_len]).to_string();
        chains.insert(end_pwd, start_pwd);
        pos += chain_len;
    }

    info!("Loaded {} chains", chains.len());

    let version_hash = hashes_data[0];
    if version_hash != VERSION {
        return Err(format!(
            "Hash file version mismatch: expected {}, got {}",
            VERSION, version_hash
        )
        .into());
    }

    let algo_len_hash = hashes_data[1] as usize;
    let algo_hash = String::from_utf8(hashes_data[2..2 + algo_len_hash].to_vec())?;

    if algo_hash.to_lowercase() != algorithm.to_lowercase() {
        return Err(format!(
            "Algorithm mismatch: table uses {}, hashes use {}",
            algorithm, algo_hash
        )
        .into());
    }

    let hash_len = match algorithm.as_str() {
        "md5" => 16,
        "sha256" => 32,
        "sha3_512" => 64,
        "scrypt" => 32,
        _ => return Err(format!("Unsupported algorithm: {}", algorithm).into()),
    };

    let allowed: Vec<u8> = (32u8..=126u8).collect();
    let mut hash_pos = 2 + algo_len_hash + 1;
    let mut found_count = 0;
    let mut total_count = 0;
    let mut output_lines = Vec::new();

    info!("Starting to crack");

    while hash_pos + hash_len <= hashes_data.len() {
        total_count += 1;
        let target_hash = &hashes_data[hash_pos..hash_pos + hash_len];
        let hash_hex = hex::encode(target_hash);

        if let Some(cache_ref) = cache {
            if let Some(entry) = cache_ref.get(&hash_hex) {
                output_lines.push(format!("{}\t{}", hash_hex, entry.value()));
                found_count += 1;
                hash_pos += hash_len;
                continue;
            }
        }

        let mut found = false;

        for i in 0..num_links {
            let mut current_hash = target_hash.to_vec();
            let mut current_pwd = reduce_hash(&current_hash, pwd_len, &allowed);

            for _ in 0..(num_links - i - 1) {
                current_hash = compute_hash(&current_pwd, &algorithm)?;
                current_pwd = reduce_hash(&current_hash, pwd_len, &allowed);
            }

            if let Some(start_pwd) = chains.get(&current_pwd) {
                let mut candidate = start_pwd.clone();

                for _ in 0..=i {
                    let candidate_hash = compute_hash(&candidate, &algorithm)?;

                    if candidate_hash == target_hash {
                        output_lines.push(format!("{}\t{}", hash_hex, candidate));
                        found_count += 1;

                        if let Some(cache_ref) = cache {
                            let _ = cache_ref.insert(
                                hash_hex.clone(),
                                candidate.clone(),
                                candidate.len() as i64,
                            );
                        }

                        found = true;
                        break;
                    }

                    candidate = reduce_hash(&candidate_hash, pwd_len, &allowed);
                }

                if found {
                    break;
                }
            }
        }

        if !found {
            output_lines.push(format!("{}\tNOT FOUND", hash_hex));
        }

        hash_pos += hash_len;
    }

    info!(
        "Cracking complete: found {}/{} passwords",
        found_count, total_count
    );

    if found_count == 0 {
        return Err("No passwords found.".into());
    }

    match out_file {
        Some(path) => {
            let mut f = File::create(path)?;
            for line in output_lines {
                writeln!(f, "{}", line)?;
            }
        }
        None => {
            for line in output_lines {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
