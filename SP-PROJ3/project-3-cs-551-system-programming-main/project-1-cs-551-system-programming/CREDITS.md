# Group Members:

#### Ankita Patra
- BU Number: B01101280
- Email: apatra@binghamton.edu

#### Himanshu Nautiyal
- BU Number: B01101396
- Email: hnautiyal@binghamton.edu

## Work Breakdown:

### Ankita Patra:
- I worked on the `gen-rainbow-table` command, including handling all its CLI options like `--in-file`, `--out-file`, `--algorithm`, `--threads`, and `--num-links`.
- I implemented the logic to read seed passwords, apply hash and reduction functions, and generate chains of a specified number of links.
- I ensured the rainbow table output followed the required binary format, including the header fields like MAGIC WORD, VERSION, ALGORITHM LENGTH, ALGORITHM NAME, PASSWORD LENGTH, CHAR SET SIZE, NUM LINKS, and ASCII OFFSET.
- I made sure the character set calculations and password length validations were accurate and properly error-checked.
- I worked on multithreaded chain generation and managed synchronization to avoid race conditions while writing output.
- I also contributed to the `dump-rainbow-table` command, formatting and printing the header fields and the chain entries in a human-readable way.
- I ran `cargo fmt`, `cargo check`, and `cargo clippy` to ensure code quality and consistency.

#### Challenges:
- Encoding and aligning all the header fields to match the project spec was difficult to get exactly right.
- Handling threads while keeping the output file consistent and avoiding data races took some trial and error.

### Himanshu Nautiyal:
- I designed the `crack` command, including all necessary CLI options like `--in-file`, `--hashes`, `--out-file` and `--threads`.
- I implemented the logic to load and parse the rainbow table and efficiently search through chains to find matching passwords for the given hashes.
- I built the reduction function pipeline and handled decoding of hexadecimal hash values for matching.
- I handled validation between the hash file and the rainbow table (like algorithm match, password length check, etc.).
- I contributed to performance tuning, testing how different values of `--threads` and `--num-links` affect generation and cracking time.
- I helped debug and test `gen-rainbow-table` and `dump-rainbow-table` to make sure output files were valid and readable.
- I maintained the `main.rs` logic to align all Project 2 subcommands and made sure argument parsing was robust.
- I also ran `cargo fmt`, `cargo check`, and `cargo clippy` to make sure everything built clean and passed linting.

#### Challenges:
- Implementing a correct and efficient hash lookup logic in `crack` that avoided unnecessary recomputation was tricky.
- Making sure algorithm mismatches or file format issues were clearly reported without panicking the program.

## Collaboration:
- We worked together to outline all subcommands.
- We frequently reviewed and tested each other's work to verify that rainbow tables, hashes, and cracking all worked together correctly.
- We wrote the `README.md` and `PERFORMANCE.md` to ensure clarity and completeness in documentation.
- We wrote this `CREDITS.md` to reflect our individual contributions and the ways we helped each other throughout Project 2.
