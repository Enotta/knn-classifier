use std::collections::HashMap;

/// Split dataframes into train and test
pub fn train_test_split(x: Vec<HashMap<String, f32>>, y: Vec<HashMap<String, f32>>, test_size: f32) -> (Vec<HashMap<String, f32>>, Vec<HashMap<String, f32>>, Vec<HashMap<String, f32>>, Vec<HashMap<String, f32>>) {
    let mut x_train = Vec::new();
    let mut y_train = Vec::new();
    let mut x_test = Vec::new();
    let mut y_test = Vec::new();

    for (x_elem, y_elem) in x.into_iter().zip(y.into_iter()) {
        if rand::random_range(0.0..=1.0) > test_size {
            x_train.push(x_elem);
            y_train.push(y_elem);
        } else {
            x_test.push(x_elem);
            y_test.push(y_elem);
        }
    }

    (x_train, y_train, x_test, y_test)
}