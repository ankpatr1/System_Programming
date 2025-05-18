use clap::{Parser, Subcommand};
use hashassin_core::{dump_hashes, gen_hashes, gen_passwords};
use std::{error::Error, fs::File, io::Write};
use tracing::info;

// This defines all the command line options using clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands, // The actual command like gen-passwords and etc ..
}

// All the other commands our program can do
#[derive(Subcommand)]
enum Commands {
    // Generate random passwords
    GenPasswords {
        // Number of characters per password by default set to 4
        #[arg(long, default_value_t = 4)]
        chars: u8,

        // Number of passwords to generate.
        #[arg(long)]
        num: usize,

        // Number of threads to use by default set to 1
        #[arg(long, default_value_t = 1)]
        threads: usize,

        // Output file. If not set prints out in the terminal
        #[arg(long)]
        out_file: Option<String>,
    },

    // Generate hashes from an input file of passwords
    GenHashes {
        // Input file containing plaintext passwords one per line
        #[arg(long = "in-file")]
        in_file: String,

        // Output file to write the hash data
        #[arg(long)]
        out_file: String,

        // Number of threads to use by default set to 1
        #[arg(long, default_value_t = 1)]
        threads: usize,

        // Hashing algorithm that can be used md5, sha256, sha3_512, scrypt
        #[arg(long)]
        algorithm: String,
    },

    // Dump hash data from a file
    #[command(name = "dump-hashes")]
    DumpHashes {
        // Input file containing the hash data
        #[arg(long = "in-file")]
        in_file: String,
    },
}

// The main function, where everything begins
fn main() -> Result<(), Box<dyn Error>> {
    // Set up logging so we can see what's happening
    tracing_subscriber::fmt::init();

    // Parse the command line arguments
    let cli = Cli::parse();

    // Figure out which command was given and run it
    match &cli.command {
        // Handles password generation
        Commands::GenPasswords {
            chars,
            num,
            threads,
            out_file,
        } => {
            info!(
                "Running gen-passwords with {} passwords, length {}, using {} threads",
                num, chars, threads
            );

            // Call our gen-password function
            let passwords = gen_passwords((*chars).into(), *num, *threads)?;

            // write to file or print to the terminal
            if let Some(file) = out_file {
                // Open file and write each password on its own line
                let mut f = File::create(file)?;
                for pwd in passwords {
                    writeln!(f, "{}", pwd)?;
                }
            } else {
                for pwd in passwords {
                    println!("{}", pwd);
                }
            }
        }

        // Handle hash generations
        Commands::GenHashes {
            in_file,
            out_file,
            threads,
            algorithm,
        } => {
            info!(
                "Generating hashes from file: {} using {} algorithm with {} threads",
                in_file, algorithm, threads
            );

            // Call our gen-hash function
            gen_hashes(in_file, out_file, algorithm, *threads)?;
        }

        // Handles dumping of our hashes
        Commands::DumpHashes { in_file } => {
            // Call our function to show hash file contents
            dump_hashes(in_file)?;
        }
    }

    // If we got here, everything worked!
    Ok(())
}
