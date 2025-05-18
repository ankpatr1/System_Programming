# Group Members:

#### Ankita Patra
- BU Number: B01101280
- Email: apatra@binghamton.edu

#### Himanshu Nautiyal
- BU Number: B01101396
- Email: hnautiyal@binghamton.edu

## Work Breakdown:

### Ankita Patra:
- I tested code for server and client as per the protocal for the further caching implementation.  
- I added caching support to the server using the `stretto` crate. This lets the system store already cracked passwords to avoid repeating the work.
- I introduced the `--cache-size` option and ensured the cracking logic checks the cache before performing any computation. I avoided using Arc<Mutex<T>> .
- I tested both no cache hits and with cache hits runs, then **measured and compared the performance**. The results showed that caching significantly reduced cracking time.
- I ran `cargo fmt`, `cargo check`, and `cargo clippy` to ensure code quality and consistency.

#### Challenges:
- I for Tested the code to implemention of  caches. 
- I faced many errors while adding caching, and fixing them took a lot of time.
- I tried using Cache<...> with the latest stretto version (0.8), but it didn’t work smoothly at first.
- I also tried wrapping the cache in Arc<Mutex<>>, but that caused deadlocks and errors in async functions.
- After reading the stretto documentation, I found out it already handles thread safety, so I removed the extra locking.
- Once I made these changes, the caching worked correctly and without issues.

### Himanshu Nautiyal:
- I worked on the server implementation starting the TCP server and handling both the `upload`and  `crack ` commands from clients.
- I worked on the logic for the client-side `upload` and `crack` crack commands, making sure they followed the expected protocol.
- I made sure the server handled input properly — reading binary data correctly, including `version`, `name,`, `size`, and `table data`.
- I implemented **multi-threaded** cracking using a semaphore to control how many computations could run at once.
- I added support for `--compute-threads`,`--async-threads`, and`--port`, so the server could be fine-tuned for performance.
- I also ran `cargo fmt`, `cargo check`, and `cargo clippy` to make sure everything built clean and passed linting.
  
#### Challenges:
- Getting the upload and crack commands to exactly match the required binary format was time-consuming.
- I had to make sure the server didn’t crash due to common async runtime mistakes like dropping a runtime in a blocking thread.

## Collaboration:
- We worked together to outline all subcommands.
- We frequently reviewed and tested each other's work to verify that rainbow tables, hashes, and cracking all worked together correctly.
- We frequently reviewed and tested each other's work to verify that Server, client , and crack all worked together correctly. 
- We verified if the password are cracked  and correctly computed using caches or not . 
- We frequently reviewed, tests and compared results with and without caching.
- We wrote the `README.md` and `PERFORMANCE.md` to ensure clarity and completeness in documentation.
- We wrote this `CREDITS.md` to reflect our individual contributions and the ways we helped each other throughout Project 3.
