use std::{fs, io::Read, path::{Path, PathBuf}};

use clap::{Parser, Subcommand};

use rhabarberbar_core::{BdxRecord, SavFile};
use tempfile::TempDir;

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

    /// Edit the .bdx files in a .sav file
    Edit {
        /// The save file to edit
        save_file: PathBuf,

        /// Save the modified data to a new file, instead of overwriting the input
        #[clap(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },

    /// Unpack a .sav file into a collection of .bdx files
    Unpack {
        /// The save file to unpack
        save_file: PathBuf,
        /// Where to store the unpacked .bdx files
        bdx_directory: PathBuf,
    },

    /// Replace the custom songs in a .sav file with a collection of .bdx files
    Pack {
        /// The save file to modify
        save_file: PathBuf,
        /// The directory containing the .bdx files to insert
        bdx_directory: PathBuf,

        /// Save the modified data to a new file, instead of overwriting the input
        #[clap(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
}

fn is_valid_filename_char(c: char) -> bool {
    !c.is_control() && "*\"/\\<>:|?".chars().all(|bad| c != bad)
}

fn unpack(save_file: impl AsRef<Path>, bdx_directory: impl AsRef<Path>) {
    fs::create_dir_all(&bdx_directory).unwrap();

    let sav = {
        let data = fs::read(save_file).unwrap();
        SavFile::from_bytes(&data)
    };

    for record in &sav.records {
        let name_raw = record.label();
        let name = name_raw
            .trim()
            .chars()
            .map(|c| if is_valid_filename_char(c) { c } else { '_' })
            .collect::<String>();

        let output_path = bdx_directory.as_ref().join(format!("{name}.bdx"));
        let bdx_bytes = record.to_bdx_bytes();
        fs::write(&output_path, &bdx_bytes).unwrap();
    }
}

fn pack(save_file: impl AsRef<Path>, bdx_directory: impl AsRef<Path>, output: Option<PathBuf>) {
    let output_file = output.unwrap_or(save_file.as_ref().to_path_buf());

    let mut sav_data = fs::read(save_file).unwrap();
    let mut sav = SavFile::from_bytes(&sav_data);

    sav.records = Vec::new();

    for file in fs::read_dir(bdx_directory).unwrap() {
        let path = file.unwrap().path();
        sav.records.push(BdxRecord::from_bdx_file(&path));
    }

    sav.records.sort_by_cached_key(|record| record.label());

    sav_data[0x190000..0x190000 + 150 * 0x8000].copy_from_slice(&sav.to_song_bytes());

    fs::write(&output_file, &sav_data).unwrap();
}

fn main() {
    let args = Cli::parse();

    match args.command {
        #[cfg(feature = "debug-commands")]
        Command::Dump { input_path } => {
            let data = fs::read(&input_path).unwrap();
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
            let data = fs::read(&input_path).unwrap();
            let sav = SavFile::from_bytes(&data);

            let record = &sav.records[index];
            let bdx_bytes = record.to_bdx_bytes();

            fs::write(&output_path, &bdx_bytes).unwrap();

            println!("Saved to {output_path:?}");
        }

        Command::Edit { save_file, output } => {
            let temp_dir = TempDir::new().unwrap();
            unpack(&save_file, temp_dir.path());
            open::that(temp_dir.path()).unwrap();
            println!("Press enter when you're done editing.");
            let _ = std::io::stdin().read(&mut [0]);
            pack(&save_file, temp_dir.path(), output);
        }

        Command::Unpack {
            save_file,
            bdx_directory,
        } => {
            unpack(&save_file, &bdx_directory);
        }

        Command::Pack {
            save_file,
            bdx_directory,
            output,
        } => {
            pack(&save_file, &bdx_directory, output);
        }
    }
}
