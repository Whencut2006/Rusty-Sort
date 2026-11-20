use crate::scanner::{categorize_entries, print_dir_name, scan_directory};

mod scanner;

fn main() {
    let mut args = std :: env :: args();
    args.next();
    match args.next() {
        Some(command) => {
            if command == "current-dir" {
                match print_dir_name() {
                    Ok(()) => {},
                    Err(error) => eprintln!("Error: {}", error)
                }
            }
            else if command == "scan" {
                match scan_directory() {
                    Ok(()) => {}
                    Err(error) => eprintln!("Error: {}", error)
                }
            }
            else if command == "categorize" {
                match categorize_entries() {
                    Ok(()) => {}
                    Err(error) => eprint!("Error: {}", error)
                }
            }
            else {
                eprintln!("Unknown command!");
            }
        }
        None => {
            eprintln!("Error: Command not provided!");
        }
    }
}
