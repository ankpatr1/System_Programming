use hashassin_core::crack;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use stretto::Cache;
use tracing::{error, info};

// Type aliases for clarity
type RainbowTables = Arc<std::sync::Mutex<HashMap<String, Vec<u8>>>>;
type SharedCache = Option<Cache<String, String>>;

// Server start function
pub fn start_server(
    address: &str,
    compute_threads: usize,
    _async_threads: usize,
    cache_size: Option<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(address)?;
    let tables: RainbowTables = Arc::new(std::sync::Mutex::new(HashMap::new()));

    // Using stretto cache directly without Arc<Mutex<>> as recommended
    let cache: SharedCache =
        cache_size.map(|size| Cache::new(512, size as i64).expect("Failed to create cache"));

    info!("Server listening on {}", address);
    info!(
        "Using compute_threads = {}, cache_size = {:?}",
        compute_threads, cache_size
    );

    for stream in listener.incoming() {
        let tables = Arc::clone(&tables);
        let cache = cache.clone();

        if let Ok(mut stream) = stream {
            thread::spawn(move || {
                let mut magic_buf = [0u8; 6];
                if let Err(e) = stream.read_exact(&mut magic_buf) {
                    error!("Failed to read magic word: {:?}", e);
                    return;
                }

                let magic_str = String::from_utf8_lossy(&magic_buf);
                match magic_str.as_ref() {
                    "upload" => {
                        if let Err(e) = handle_upload(&mut stream, &tables) {
                            error!("Upload failed: {:?}", e);
                        }
                    }
                    "crack\u{0}" => {
                        if let Err(e) = handle_crack_sync(&mut stream, &tables, cache.clone()) {
                            error!("Crack failed: {:?}", e);
                        }
                    }
                    _ => {
                        error!("Unknown command: {:?}", magic_buf);
                    }
                }
            });
        }
    }

    Ok(())
}

// Upload handler
fn handle_upload(stream: &mut TcpStream, tables: &RainbowTables) -> std::io::Result<()> {
    let mut version = [0u8; 1];
    let mut name_len = [0u8; 1];

    stream.read_exact(&mut version)?;
    stream.read_exact(&mut name_len)?;

    let mut name = vec![0u8; name_len[0] as usize];
    stream.read_exact(&mut name)?;

    let mut size = [0u8; 8];
    stream.read_exact(&mut size)?;

    let payload_len = u64::from_be_bytes(size);
    let mut payload = vec![0u8; payload_len as usize];
    stream.read_exact(&mut payload)?;

    let name_str = String::from_utf8_lossy(&name).to_string();
    info!(
        "Received rainbow table '{}' ({} bytes)",
        name_str, payload_len
    );

    if let Ok(mut map) = tables.lock() {
        map.insert(name_str, payload);
    }

    Ok(())
}

// Crack handler
fn handle_crack_sync(
    stream: &mut TcpStream,
    tables: &RainbowTables,
    cache: SharedCache,
) -> std::io::Result<()> {
    let mut version = [0u8; 1];
    let mut size = [0u8; 8];

    stream.read_exact(&mut version)?;
    stream.read_exact(&mut size)?;

    let payload_len = u64::from_be_bytes(size);
    let mut payload = vec![0u8; payload_len as usize];
    stream.read_exact(&mut payload)?;

    info!("Received crack request ({} bytes)", payload_len);

    let table_data: Vec<Vec<u8>> = {
        let guard = tables.lock().unwrap();
        guard.values().cloned().collect()
    };

    std::fs::write("temp_hashes.bin", &payload)?;
    let mut combined_results = Vec::new();
    let mut found = false;

    for table in &table_data {
        std::fs::write("temp_table.rainbow", table)?;

        match crack(
            "temp_table.rainbow",
            "temp_hashes.bin",
            Some("cracked.txt"),
            1,
            cache.as_ref(),
        ) {
            Ok(_) => {
                let result = std::fs::read_to_string("cracked.txt").unwrap_or_default();
                combined_results.push(result);
                found = true;
                break;
            }
            Err(e) => {
                info!("Table didn't find matches: {:?}", e);
            }
        }
    }

    if found {
        stream.write_all(combined_results.concat().as_bytes())?;
    } else {
        stream.write_all(b"No passwords cracked\n")?;
    }

    Ok(())
}
