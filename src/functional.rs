use std::{collections::HashMap, io, path::Path};
use std::error::Error;

/// Read single line from console, return trimmed String
pub fn input(annotation: String) -> Result<String, Box<dyn Error>> {
    let mut str_path = String::new();
    println!("{}", annotation);
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

/// Split dataframe into features dataframe and target one
pub fn split_features_target(dataframe: Vec<HashMap<String, f32>>, k: String) -> (Vec<HashMap<String, f32>>, Vec<HashMap<String, f32>>) {
    if !dataframe.iter().all(|row| row.contains_key(&k)) {
        panic!("Can't split dataframe. Not all rows contain target key '{}'", k);
    }

    let mut x = Vec::with_capacity(dataframe.len());
    let mut y = Vec::with_capacity(dataframe.len());

    for mut row in dataframe.into_iter() {
        let target_value = row.remove(&k).unwrap();
        
        let mut y_row = HashMap::new();
        y_row.insert(k.clone(), target_value);
        
        x.push(row);
        y.push(y_row);
    }

    (x, y)
}