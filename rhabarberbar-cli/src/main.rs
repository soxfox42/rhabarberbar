use clap::{Parser, Subcommand};

use rhabarberbar_core::SavFile;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// DEBUG: Print information about the contents of the input .sav
    Dump { input_path: String },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Dump { input_path } => {
            let data = std::fs::read(&input_path).unwrap();
            let sav = SavFile::from_bytes(&data);

            for (i, record) in sav.records.iter().enumerate().filter(|(_, r)| r.used()) {
                println!("Slot {i}");
                println!("Label: {}", &record.label());
                println!("Contributor: {}", &record.contributor());

                println!("---");
            }
        }
    }
}
