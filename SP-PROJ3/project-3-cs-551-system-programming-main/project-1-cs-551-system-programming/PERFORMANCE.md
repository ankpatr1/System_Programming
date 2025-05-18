# PERFORMANCE.md

* Experimental Setup

- Machine : MacBook Pro M1, 8-core CPU, 16GB RAM  
- OS : macOS Ventura 13.6.5  
- Rust version: rustc 1.85.1  
- Build: " cargo build --release"  
- Dataset: 1,000,00 randomly generated passwords using  "gen-passwords"
- Thread counts tested : 1, 4, 6, 8  
- Hash algorithms tested : md5, sha256, sha3_512  
- Password lengths tested : 5, 10, 15, 20 


## Password Length vs Execution Time (md5)

Measured using "gen-rainbow-table" with:

- 1,000,000 passwords
- links : 1,5,15, 30
- password_length : 5, 10, 15, 20
- threads: 1,4,6,8
- Algorithm: md5, sha256, sha3_512  

| Password Length | md5 (sec) | sha256 (sec) | sha3_512 (sec) |
| --------------- | --------- | ------------ | --------------- |
| 5               | 6.17      | 12.05        | 40.39           |
| 10              | 6.18      | 12.39        | 40.30           |
| 15              | 6.26      | 12.18        | 40.39           |
| 20              | 6.32      | 12.44        | 40.30           |


### Observations

-> Time increases slightly from length 5 to 10, then stays flat — password length has minimal impact.
-> md5 is fastest (~6s), sha256 is moderate (~12s), and sha3_512 is slowest (~40s).
-> Algorithm choice matters more than password length.
Use:
  -> md5 for speed,
  -> sha256 for balance,
  -> sha3_512 for strong security.



* Algorithm Performance Comparison



| Password Length | MD5 (sec) | SHA256 (sec) | SHA3_512 (sec) |
|------------------|-----------|---------------|----------------|
| 5                | 6.17      | 12.05         | 40.39          |
| 10               | 6.18      | 12.39         | 40.30          |
| 15               | 6.26      | 12.18         | 40.39          |
| 20               | 6.32      | 12.44         | 40.10          |

* Observations:
- MD5 is consistently the fastest due to its smaller digest size and lightweight computation.
- SHA256 offers a good trade-off between speed and cryptographic strength.
- SHA3_512 is the slowest, but benefits most from multithreading and provides the strongest security.
All algorithms see significant performance gains up to 4 threads, after which the improvements taper due to CPU/thread overhead.

*  Thread Scaling Performance



| Threads | MD5 (sec) | SHA256 (sec) | SHA3\_512 (sec) |
| ------- | --------- | ------------ | --------------- |
| 1       | 20.45     | 21.31        | 68.12           |
| 4       | 12.40     | 12.17        | 39.71           |
| 6       | 7.48      | 7.66         | 22.28           |
| 8       | 5.98      | 6.12         | 18.63           |


**Observation**:
- Performance improves noticeably as thread count increases.
- The biggest gain is from 1 → 4 and 4 → 6 threads.
- Gains begin to plateau by 8 threads, likely due to CPU  limits or thread scheduling overhead.


## Commands
*  Run below before password generation : 
  
-> cargo fmt - Format code to style guidelines
-> cargo check - Fast error/warning detection
-> cargo build - Compile your code
-> cargo clippy - Lint code and catch subtle issues

#  Project 1 :
--------------------
* cargo run gen-passwords --chars 6 --num 100000 --threads 4 --out-file passwords.txt 
  
* cargo run gen-hashes --in-file passwords.txt --out-file hashes.bin --threads 4 --algorithm md5
  
* cargo run dump-hashes --in-file hashes.bin

# Project 2 :
--------------------------
# Rainbow table generation
cargo run gen-rainbow-table --in-file passwords.txt --out-file table.rainbow --algorithm md5 --num-links 500 --threads 2

# Dump rainbow table generation
cargo run dump-rainbow-table --in-file table.rainbow

# Password cracking
cargo run crack --in-file table.rainbow --hashes hashes.bin

-> cargo doc --document-private-items --no-deps   - Generate full project documentation