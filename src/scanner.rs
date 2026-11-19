use std::env;
use std::path::PathBuf;

pub fn print_dir_name() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}


pub fn scan_directory() -> std :: io :: Result<()>{
    let dir_iter = std :: fs :: read_dir(std :: env :: current_dir().unwrap()).unwrap();
    for entry in dir_iter {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        println!("{}", file_name.display());
    }

    Ok(())

}