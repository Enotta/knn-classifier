mod functional;
mod ml;

use functional::{input, read_csv, split_features_target};
use ml::train_test_split;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str_path = input("File name:".to_string())?;
    let file = read_csv(&str_path)?;
    
    let (x, y) = split_features_target(file, "Outcome".to_string());
    let (x_train, y_train, x_test, y_test) = train_test_split(x, y, 0.2);

    println!("{} {} {} {}", x_train.len(), y_train.len(), x_test.len(), y_test.len());

    Ok(())
}