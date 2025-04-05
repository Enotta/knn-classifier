use std::{collections::HashMap, io, path::Path};
use std::error::Error;

/// Read single line from console, return trimmed String
pub fn input() -> Result<String, Box<dyn Error>> {
    let mut str_path = String::new();
    let _ = io::stdin().read_line(&mut str_path)?;

    Ok(str_path.trim().to_string())
}

/// Take &str path, return file reader
pub fn read_csv(str_path: &str) -> Result<Vec<HashMap<String, f32>>, Box<dyn Error>> {
    let path = Path::new(&str_path);
    let mut file = csv::Reader::from_path(path)?;

    let mut result = Vec::<HashMap<String, f32>>::new();
    for row in file.deserialize() {
        let record: HashMap<String, f32> = row?;
        result.push(record);
    }


    Ok(result)
}