mod hashing;
mod scanner;
mod filter;
mod quarantine;
mod report;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "duupli", version = "0.1", author = "You")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find duplicate files using hashes
    Same {
        /// Path to scan
        path: String,

        /// Quarantine duplicates instead of just reporting
        #[arg(short, long)]
        quarantine: bool,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Same { path, quarantine } => {
            let files = scanner::scan_dir(&path);
            let filtered = filter::apply_filters(files);
            let duplicates = hashing::find_duplicates(filtered);

            if quarantine {
                quarantine::quarantine_files(&duplicates);
            }

            report::generate_report(&duplicates);
            println!(" Done. Found {} duplicate sets.", duplicates.len());
            if quarantine {
                println!(" Quarantined files.");
            }
        }
    }
}
