mod models{
    pub mod dataset;
    pub mod gradient_descent;
    use std::fs;
}

pub mod linear_regression{
    struct LinearRegression{
        dataset: Dataset,
        cost: f32,
        w0: f32,
        w1: f32,
        data: Vec<i32>,
        value: Vec<i32>,
        gradient: Gradient
    }

    impl LinearRegression{

        pub fn new(path: &str, w0: f32, w1: f32) -> Self {
            LinearRegression {
                dataset: Dataset::new(path),
                w0,
                w1,
                cost: f32::new()
            }
        }

        pub fn train_model(&mut self, epochs: i32, alpha: i32) {
            self.dataset.make_dataset();
            self.gradient = Gradient::new(epochs, alpha);

            let aux:(f32, f32, [f32;10]) = self.gradient.gradient_descent(&self.w0, &self.w1,
                                                                          &self.dataset.itens, 
                                                                          &self.dataset.predict);

            self.w0   = aux.0;
            self.w1   = aux.1;
            self.cost = aux.2[9];

            fs::write(
                "model.txt",
                format!(
                    
                )
            )
        }

        pub fn predict(&self, x: i32, y: i32) -> i32 {
            return ((&self.w1*x) + &self.w0);
        }
    }

}