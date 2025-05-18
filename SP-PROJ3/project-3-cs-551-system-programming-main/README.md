# Hashassin

This project is a system programming project to show multi-threading, file I/O, and hash algorithm implementations like MD5, SHA-256, SHA3-512, and Scrypt.

It provides a command-line tool for:
- Generating random passwords
- Generating hashes from passwords using various algorithms
- Dumping hash files to readable format
- Creating rainbow tables from password seeds
- Cracking hashes using rainbow tables
- Implementing a TCP client and server architecture
- Uploading and storing rainbow tables over TCP
- Performing distributed password cracking via TCP with concurrent client support

## Directory Structure:
  hashassin/
-  ├── Cargo.toml
-  ├── cli/
-  │   └── src/
-  │       └── main.rs    
-  ├── core/
-  │   └── src/
-  │       └── lib.rs
-  ├── client/
-  │   └── src/
-  │       └── lib.rs    
-  ├── server/
-  │   └── src/
-  │       └── lib.rs      
-  ├── HONESTY.md            
-  ├── CREDITS.md              
-  ├── README.md
-  ├── PERFORMANCE.md        

#### This project is implemented in Rust and consists of two components:
- *Core Library (hashassin-core)* - Implements the core functionalities for password generation, hashing and dumping hash into plaintext, rainbow table creation, dump rainbow table and cracking
  
- *CLI Application (hashassin-cli)* - Provides a command-line interface to interact with the core library
  
- *Client Library (hashassin-client)*: Exposes functionality for TCP-based operations, including "upload" and "crack".

- *Server Library (hashassin-server)*: Implements the TCP server that handles incoming upload and crack requests, manages in‑memory storage of tables, and coordinates the cracking workflow.  

### Core Features

### ✅ Project 1 Features
Generate Passwords (gen-passwords)
- Generate random ASCII-based passwords
- Multi-threaded password generation
- Save passwords to a specified output file

Hash Passwords (gen-hashes)
- Read passwords from the given input file and hash them
- Support hashing algorithms: MD5, SHA-256, SHA3-512, and Scrypt
- Multi-threaded hashing for performance

Dump Hashes (dump-hashes)
- Extract hashed passwords from a generated hash file
- Output metadata: Version, algorithm name, password length

### ✅ Project 2 Features
Generate Rainbow Table (gen-rainbow-table)
- Builds a rainbow table from an input password file
- Supports hashing with MD5, SHA-256, SHA3-512
- Accepts parameters like links, threads, out-file, algorithm, in-file
- Outputs a binary `.rainbow` or `.rt` file with reduced storage format
- Output includes a binary header with metadata like algorithm, charset, and chain length

Dump Rainbow Table (dump-rainbow-table)
- Reads and prints the content of a rainbow table in human-readable format
- Print all metadata from a `.rainbow` table including version, algorithm, charset, and link information
- List all chain start and end points for easier debugging

Crack (crack)
- Cracks hashes using a precomputed rainbow table
- Matches hash values to original passwords via chain reversal
- Efficient lookup with consistent formatting
- Reports passwords or `NOT FOUND` if the hash is not in the coverage

### ✅ Project 3 Features

Start Server (server)
- Launch a `TCP` server on a specified address and port (By default it uses `127.0.0.1:2025` )
- Accepts two commands:
      * upload: receives and stores one or more rainbow tables in memory 
      * crack: Accepts a binary hash file and uses all uploaded tables to attempt cracking then return the results in Project 2's format
- Supports configurable compute threads and async runtime threads for handling client connections

Upload Table (client upload)
- Sends a .rainbow file to the server over TCP using upload command
- Assigns a user defined name to each uploaded table for organized storage
- Uploaded Tables are stored in server memory and remain available until the server shuts down

Crack Over TCP (client crack)
- Sends a binary hashes file to the server using the crack command
- The server cracks the hashes using all currently uploaded rainbow tables
- Receive and save cracked results to a `.txt` file

## How to Run the project:

#### NOTE: To update rust to latest stable version. RUN `rustup update stable` (Will give you the latest stable version: rustc 1.85.1)

- STEP 1: git clone https://github.com/2025-Spring-CS-551/project-1-cs-551-system-programming.git

- STEP 2: cargo run gen-passwords --num 10 --chars 8 --out-file passwords.txt

  Options:

  - num: Number of passwords to generate (required)
  
  - chars: Number of characters per password (default: 4)
  
  - threads: Number of threads to use (default: 1)
  
  - out-file: Output file (optional). If omitted, passwords are printed in the terminal

- STEP 3: cargo run gen-hashes --in-file passwords.txt --out-file hashes.bin --algorithm sha256 --threads 2

  Options:
  
  - in-file: Password.txt. The password file created by gen-password function
  
  - algorithm: Mention what algorithm you want to use. Options: MD5, SHA-256, SHA3-512, and Scrypt
  
  - threads: Number of threads to use (default: 1)
  
  - out-file: Output file (optional). File to store hashed passwords

- STEP 4: cargo run dump-hashes --in-file hashes.bin

  Options:
  
  - in-file: hashes.bin. The hash file created by the gen-hashes function

- STEP 5: cargo run gen-rainbow-table --in-file passwords.txt --out-file table.rainbow --algorithm md5 --num-links 500 --threads 4

  Options:
  
  - in-file: Seed password file
  
  - algorithm: Hashing algorithm to use
  
  - num-links: Number of hash-reduce links per chain
  
  - threads: Number of threads to use for generation
  
  - out-file: Output file where the table will be saved

- STEP 6: cargo run dump-rainbow-table --in-file table.rainbow

  Options:
  
  - in-file: Path to `.rainbow` file created by gen-rainbow-table

- STEP 7: cargo run crack --in-file table.rainbow --hashes hashes.bin --out-file cracked.txt --threads 4

  Options:

  - --in-file: Rainbow table used to reverse the hashes

  - --hashes-file: Hash file to be cracked

  - --out-file: File where cracked passwords (or NOT FOUND) will be saved

  - --threads: Number of threads to parallelize the cracking process

- STEP 8:

cargo run server --bind 127.0.0.1 --port 2025 --compute-threads 4 --async-threads 4 --cache-size 8192

  Options:

  - --bind: IP address to bind (default: 127.0.0.1)

  - --port: TCP port to listen on (default: 2025)

  - --compute-threads: maximum concurrent cracking threads (default: 1)

  - --async-threads: size of the async runtime thread pool (default: 1)

  - --cache-size: optional LRU cache capacity for previously cracked passwords (max i32 bytes)

- STEP 9:cargo run client-upload --server 127.0.0.1:2025 --in-file table.rainbow --name demo
  
  options:

  - --server : target server address (e.g., 127.0.0.1:2025)

  - --in-file : path to the .rainbow file

  - --name : label under which the table is stored

- STEP 10 : cargo run client-crack --server 127.0.0.1:2025 --in-file hashes.bin --out-file cracked.txt
  
  Options:

  - --server : target server address
  
  - --in-file : path to the binary hashes file (Project 2 format)

  - --out-file : (optional) file to write results; defaults to stdout

## Crates used in our project

#### Core Functionality:
- hex
- rand
- md5
- sha256
- sha3_512
- scrypt
- tracing

#### CLI-Specific:
- clap
- tracing-subscriber

#### Server‑Specific:
- tokio
- tracing
- stretto 

#### Workspace:
- hashassin_core

### Grading Rubric Items Attempted
- [x] `CREDITS.md` not updated (avoid -1,000 penalty) 
- [x]  `HONESTY.md` not included (avoid -1,000 penalty) 
- [x] `README.md` not included (avoid -1,000 penalty)
- [x] `cargo fmt` compliance (avoid -1,000 penalty)  
- [x] `correct binary/library` Program does not compile (avoid -1,000 penalty)
- [x] `Program compiles` (25 points)
- [x] All `server` functionality implemented (--bind, --port, --compute-threads, --async-threads, --cache-size ) (35 points)  
- [x] All `client` functionality implemented(30 points) 
      *  `upload` (--server, --in-file, --name)
      *  `crack` (--server, --in-file, --out-file )
- [x] `Comprehensive documentation` functionality implemented (5 points)
- [x] `No warnings from cargo check`(2.5 points)
- [x] `No warnings from cargo clippy`(2.5 points)
- [x] `Proper error handling` (2.5 points) 
- [x] No `unwrap`/`expect` (denied via `#![deny(clippy::unwrap_used)]`) (2.5 points)  
- [x] Proper logging with `tracing` (2.5 points)  
- [x] `PERFORMANCE.md` (10 points) 
