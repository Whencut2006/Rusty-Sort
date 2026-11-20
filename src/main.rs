use crate::scanner::{categorize_entries, print_dir_name, scan_directory};
use crate::cli::{parse, Commands};

mod scanner;
mod cli;

fn main() {
    // let args = parse();

    match &parse().command {
        Commands ::CurrentDir => {
            match print_dir_name() {
                Ok(()) => {},
                Err(error) => {eprintln!("Error: {}", error);}
            }
        }

        Commands :: Scan => {
            match scan_directory() {
                Ok(()) => {},
                Err(error) => {eprintln!("Error: {}", error);}
            }
        }
        Commands :: Categorize => {
            match categorize_entries() {
                Ok(()) => {},
                Err(error) => {eprintln!("Error: {}", error);}
            }
        }

    }

}
