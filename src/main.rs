mod functional;

use functional::{input, read_csv};

use std::error::Error;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let str_path = input()?;
    let file = read_csv(&str_path)?;
    
    for i in 0..file.len() {
        for key in file[i].keys().sorted() {
            print!("{} {} ", key, file[i].get(key).unwrap())
        }
        println!()
    }
    Ok(())
}