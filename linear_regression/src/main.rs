mod models{
    pub mod dataset;
}

use crate::models::dataset::dataset::Dataset;

fn main() {
    let mut test: Dataset = Dataset::new(r"C:\Users\joaop\Documents\projects\rust-studies\linear_regression\src\models\dataset.txt");
    test.make_dataset();

    // let a: f32 = 1.5;
    // let b: f32 = f32::powf(a,2.0);

    // println!("pow of a = {b}");
}
