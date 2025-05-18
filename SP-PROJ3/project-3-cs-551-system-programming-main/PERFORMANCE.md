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
- In **project3** ,we building a TCP client and server. 
- We checked how fast our server runs with and without using a cache and measure the difference.  


# Project -2     

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

# Project 3: 

* the number of clients that are connected vs. the number of async threads and the number of compute threads(no caches VS with caches):

|Clients(N)| Async Threads | Compute Threads | Time (No Cache) | Throughput (No Cache) | Latency (No Cache) | Time (Cache) | Throughput (Cache) | Latency (Cache) |
| ------- | ------------- | --------------- | --------------- | --------------------- | ------------------ | ------------ | ------------------ | --------------- |
| 10      | 4             | 4               | 7.44 s          | 1.34/s                | 0.74 s             | 0.62 s       | 15.93/s            | 0.067 s         |
| 20      | 8             | 6               | 0.61 s          | 32.84/s               | 0.030 s            | 0.87 s       | 22.84/s            | 0.043 s         |
| 40      | 12            | 8               | 1.18 s          | 33.84/s               | 0.030 s            | 1.84 s       | 21.66/s            | 0.046 s         |
| 60      | 14            | 10              | 1.82 s          | 32.99/s               | 0.030 s            | 2.57 s       | 23.32/s            | 0.042 s         |
| 80      | 16            | 12              | 2.25 s          | 35.49/s               | 0.028 s            | 3.45 s       | 23.12/s            | 0.043 s         |

Calculation of total time and Throughput and Avg.Latency :
STEP 1: **Async Threads and Compute Threads** : cargo run -- server --bind 127.0.0.1 --port 2025 --compute-threads 4 --async-threads  ( without caches)
                                                cargo run -- server --bind 127.0.0.1 --port 2025 --compute-threads 4 --async-threads 4 --cache-size 8192 (with caches 8GB)
STEP 2: **Throughput (clients/s)** : N/T : Client/ Time
STEP 3: **Avg. Latency** : T/N

**Observation**:
* When you don’t give the server enough threads, it slows down badly as more clients connect.
* Using about 6 compute threads and 8 async threads hits the sweet spot: the server runs near top speed and keeps responses fast and steady, even with lots of clients.
* We tested above details using caches size using : 8192 (8GB)
* Cracking passwords got much faster once caching was added.
* Even when 80 clients connected at the same time, the server stayed fast — each client waited only around 0.036 seconds.
* The number of clients handled per second (throughput) was higher with caching — going up to about 29 clients per second.
* after testing we see The system handled more clients with minimal slowdown. according above table around 6–10 compute threads and 8–14 async threads gave the best speed.

-> NOTE : using stretto with cache.  

## Commands
*  Run below before password generation : 
  
-> cargo fmt - Format code to style guidelines
-> cargo check - Fast error/warning detection
-> cargo build - Compile your code
-> cargo clippy - Lint code and catch subtle issues

#  Project 1 :
--------------------
* cargo run gen-passwords --chars 6 --num 10 --threads 4 --out-file passwords.txt 
  
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

# Project 3 :
--------------------------
# Start the server in the new terminal 

 cargo run server --bind 127.0.0.1 --port 2025 --compute-threads 4 --async-threads 4

 with caches : cargo run server --bind 127.0.0.1 --port 2025 --compute-threads 4 --async-threads 4 --cache-size 8192

# Upload table:
  
 cargo run client-upload --server 127.0.0.1:2025 --in-file table.rainbow --name demo

# Crack hashes: 
  
  cargo run client-crack --server 127.0.0.1:2025 --in-file hashes.bin --out-file crack.txt ( No caches)
  
  With Cache : 

    cargo run client-crack --server 127.0.0.1:2025 --in-file hashes.bin --out-file cracked.txt

-> cargo doc --document-private-items --no-deps   - Generate full project documentation