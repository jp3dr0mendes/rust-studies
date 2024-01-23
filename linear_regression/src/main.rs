mod models{
    pub mod linear_regression;
}

// use crate::models::dataset::dataset::Dataset;
use crate::models::linear_regression::linear_regression::LinearRegression;
fn main() {
    // let mut test: Dataset = Dataset::new(r"C:\Users\joaop\Documents\projects\rust-studies\linear_regression\src\models\dataset.txt");
    let mut Model: LinearRegression = LinearRegression::new(r"C:\Users\joaop\Documents\projects\rust-studies\linear_regression\src\dataset.txt",
                                                            0.0, 0.0);

    Model.train_model(100, 0.5);
}
