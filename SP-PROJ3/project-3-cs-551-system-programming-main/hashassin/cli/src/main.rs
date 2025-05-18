use clap::{Parser, Subcommand};
use hashassin_client::{crack as client_crack, upload};
use hashassin_core::{
    crack, dump_hashes, dump_rainbow_table, gen_hashes, gen_passwords, gen_rainbow_table,
};
use hashassin_server::start_server;
use std::{error::Error, fs::File, io::Write};
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenPasswords {
        #[arg(long, default_value_t = 4)]
        chars: u8,
        #[arg(long)]
        num: usize,
        #[arg(long, default_value_t = 1)]
        threads: usize,
        #[arg(long)]
        out_file: Option<String>,
    },
    GenHashes {
        #[arg(long = "in-file")]
        in_file: String,
        #[arg(long)]
        out_file: String,
        #[arg(long, default_value_t = 1)]
        threads: usize,
        #[arg(long)]
        algorithm: String,
    },
    DumpHashes {
        #[arg(long = "in-file")]
        in_file: String,
    },
    GenRainbowTable {
        #[arg(long = "in-file")]
        in_file: String,
        #[arg(long)]
        out_file: String,
        #[arg(long, default_value_t = String::from("md5"))]
        algorithm: String,
        #[arg(long, default_value_t = 5)]
        num_links: usize,
        #[arg(long, default_value_t = 1)]
        threads: usize,
    },
    DumpRainbowTable {
        #[arg(long = "in-file")]
        in_file: String,
    },
    Crack {
        #[arg(long = "in-file", alias = "table-file")]
        in_file: String,
        #[arg(long = "hashes")]
        hashes: String,
        #[arg(long)]
        out_file: Option<String>,
        #[arg(long, default_value_t = 1)]
        threads: usize,
    },
    Server {
        #[arg(long, default_value = "127.0.0.1")]
        bind: String,
        #[arg(long, default_value_t = 2025)]
        port: u16,
        #[arg(long, default_value_t = 1)]
        compute_threads: usize,
        #[arg(long, default_value_t = 1)]
        async_threads: usize,
        #[arg(long)]
        cache_size: Option<i32>,
    },
    ClientUpload {
        #[arg(long)]
        server: String,
        #[arg(long = "in-file")]
        in_file: String,
        #[arg(long)]
        name: String,
    },
    ClientCrack {
        #[arg(long)]
        server: String,
        #[arg(long = "in-file")]
        in_file: String,
        #[arg(long)]
        out_file: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenPasswords {
            chars,
            num,
            threads,
            out_file,
        } => {
            info!(
                "Generating {} passwords of length {} using {} threads",
                num, chars, threads
            );
            let passwords = gen_passwords((*chars).into(), *num, *threads)?;
            if let Some(file) = out_file {
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
        Commands::GenHashes {
            in_file,
            out_file,
            threads,
            algorithm,
        } => {
            info!(
                "Generating hashes from '{}' using {} algorithm",
                in_file, algorithm
            );
            gen_hashes(in_file, out_file, algorithm, *threads)?;
        }
        Commands::DumpHashes { in_file } => {
            dump_hashes(in_file)?;
        }
        Commands::GenRainbowTable {
            in_file,
            out_file,
            algorithm,
            num_links,
            threads,
        } => {
            info!("Generating rainbow table from '{}'", in_file);
            gen_rainbow_table(in_file, out_file, algorithm, *num_links, *threads)?;
        }
        Commands::DumpRainbowTable { in_file } => {
            dump_rainbow_table(in_file)?;
        }
        Commands::Crack {
            in_file,
            hashes,
            out_file,
            threads,
        } => {
            crack(in_file, hashes, out_file.as_deref(), *threads, None)?;
        }
        Commands::Server {
            bind,
            port,
            compute_threads,
            async_threads,
            cache_size,
        } => {
            let address = format!("{}:{}", bind, port);
            info!(
                "Starting server at {} with compute_threads={}, async_threads={}, cache_size={:?}",
                address, compute_threads, async_threads, cache_size
            );
            start_server(&address, *compute_threads, *async_threads, *cache_size)?;
        }
        Commands::ClientUpload {
            server,
            in_file,
            name,
        } => {
            info!("Uploading '{}' as '{}' to {}", in_file, name, server);
            upload(server, in_file, name)?;
        }
        Commands::ClientCrack {
            server,
            in_file,
            out_file,
        } => {
            info!("Submitting crack job to {}", server);
            let result = client_crack(server, in_file)?;
            if let Some(path) = out_file {
                let mut f = File::create(path)?;
                f.write_all(&result)?;
            } else {
                println!("{}", String::from_utf8_lossy(&result));
            }
        }
    }

    Ok(())
}
