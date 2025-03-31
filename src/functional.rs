use std::{io, fs::File, path::Path};
use csv::Reader;
use std::error::Error;

/// Read single line from console, return trimmed String
pub fn input() -> Result<String, Box<dyn Error>> {
    let mut str_path = String::new();
    let _ = io::stdin().read_line(&mut str_path)?;

    Ok(str_path.trim().to_string())
}

/// Take string path? return file reader
pub fn read_csv(str_path: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let path = Path::new(&str_path);
    let file = csv::Reader::from_path(path)?;

    Ok(file)
}