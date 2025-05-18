# Hashassin

This project is a system programming project to show multi-threading, file I/O, and hash algorithm implementations like MD5, SHA-256, SHA3-512, and Scrypt
It provides a command-line tool for:
- Generating random passwords
- Generating hashes from passwords using various algorithms
- Dumping hash files to readable format
- Creating rainbow tables from password seeds
- Cracking hashes using rainbow tables

## Directory Structure:
  hashassin/
-  ├── Cargo.toml
-  ├── cli/
-  │   └── src/
-  │       └── main.rs    
-  ├── core/
-  │   └── src/
-  │       └── lib.rs     
-  ├── HONESTY.md            
-  ├── CREDITS.md              
-  ├── README.md
-  ├── PERFORMANCE.md        

#### This project is implemented in Rust and consists of two components:
- *Core Library (hashassin-core)* - Implements the core functionalities for password generation, hashing and dumping hash into plaintext, rainbow table creation, dump rainbow table and cracking
- *CLI Application (hashassin-cli)* - Provides a command-line interface to interact with the core library

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

## Crates used in our project

#### Core Functionality:
- hex
- rand
- md5
- sha2
- sha3
- scrypt
- tracing

#### CLI-Specific:
- clap
- tracing-subscriber

#### Workspace:
- hashassin_core

### Grading Rubric Items Attempted  
- [x] `Program compiles` (25 points)  
- [x] All `gen-rainbow-table` functionality implemented(--algorithm,--in-file, --out-file, --threads, --num-links) (25 points)  
- [x] All `dump-rainbow-table` functionality implemented(--in-file, Output conforms to specification) (15 points)  
- [x] All `crack` functionality implemented(Output conforms to spec,--in-file, --out-file, --threads, --hashes) (25 points)  
- [x] `Comprehensive documentation` functionality implemented(5 points)
- [x] `No warnings from cargo check`(2.5 points)
- [x] `No warnings from cargo clippy`(2.5 points)
- [x] `Support passwords that are greater than 10 characters`(5 points)
- [x] `Handle character set sizes that are greater than 255`(5 points)
- [x] `Proper error handling` (2.5 points)  
- [x] `Support additional hashing algorithms beside md5`(0.1 points)
- [x] No `unwrap`/`expect` (denied via `#![deny(clippy::unwrap_used)]`) (2.5 points)  
- [x] Proper logging with `tracing` (2.5 points)  
- [x] `PERFORMANCE.md` (15 points) 
- [x] `cargo fmt` compliance (avoid -1,000 penalty)  
- [x] `CREDITS.md` not updated (avoid -1,000 penalty)  
- [x] `README.md` not included (avoid -1,000 penalty) 