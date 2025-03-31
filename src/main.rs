mod functional;

use functional::{input, read_csv};

use std::{collections::HashMap, error::Error};

type Record = HashMap<String, String>;

fn main() -> Result<(), Box<dyn Error>> {
    let str_path = input()?;
    let mut file = read_csv(&str_path)?;
    
    for result in file.deserialize(){
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
