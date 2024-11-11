use std::path::PathBuf;

use clap::{Parser, Subcommand};

use rhabarberbar_core::SavFile;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[cfg(feature = "debug-commands")]
    /// DEBUG: Print information about the contents of the input .sav
    Dump { input_path: PathBuf },

    #[cfg(feature = "debug-commands")]
    /// DEBUG: Export a single bdx file from the input .sav by index
    DumpOne {
        input_path: PathBuf,
        index: usize,
        output_path: PathBuf,
    },

    /// Unpack a .sav file into a collection of .bdx files
    Unpack {
        input_path: PathBuf,
        output_dir: PathBuf,
    },
}

fn is_valid_filename_char(c: char) -> bool {
    c.is_ascii() && !c.is_ascii_control() && c != '/' && c != '\\'
}

fn main() {
    let args = Cli::parse();

    match args.command {
        #[cfg(feature = "debug-commands")]
        Command::Dump { input_path } => {
            let data = std::fs::read(&input_path).unwrap();
            let sav = SavFile::from_bytes(&data);

            for (i, record) in sav.records.iter().enumerate() {
                println!("Song #{i}");
                println!("Label: {}", &record.label());
                println!("Contributor: {}", &record.contributor());

                println!("---");
            }
        }

        #[cfg(feature = "debug-commands")]
        Command::DumpOne {
            input_path,
            index,
            output_path,
        } => {
            let data = std::fs::read(&input_path).unwrap();
            let sav = SavFile::from_bytes(&data);

            let record = &sav.records[index];
            let bdx_bytes = record.to_bytes_bdx();

            std::fs::write(&output_path, &bdx_bytes).unwrap();

            println!("Saved to {output_path:?}");
        }

        Command::Unpack {
            input_path,
            output_dir,
        } => {
            std::fs::create_dir_all(&output_dir).unwrap();

            let data = std::fs::read(&input_path).unwrap();
            let sav = SavFile::from_bytes(&data);

            for record in &sav.records {
                let name_raw = record.label();
                let name = name_raw
                    .chars()
                    .map(|c| if is_valid_filename_char(c) { c } else { '_' })
                    .collect::<String>();

                let mut output_path = output_dir.join(format!("{name}.bdx"));
                let mut counter = 1;
                while output_path.exists() {
                    counter += 1;
                    output_path.set_file_name(format!("{name} ({counter}).bdx"));
                }

                let bdx_bytes = record.to_bytes_bdx();
                std::fs::write(&output_path, &bdx_bytes).unwrap();
            }
        }
    }
}
