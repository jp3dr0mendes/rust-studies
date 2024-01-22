use std::io::{self, BufRead};
mod models{
    pub mod dataset;
}

use crate::models::dataset::dataset::Dataset;

fn main() {
    let mut test: Dataset = Dataset::new(r"C:\Users\joaop\Documents\projects\rust-studies\linear_regression\src\models\dataset.txt");
    test.make_dataset();
}
