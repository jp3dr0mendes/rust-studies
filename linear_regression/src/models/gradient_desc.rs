pub mod gradient{

    pub struct Gradient{
        epochs: i32,
        alpha: f32,
    }

    impl Gradient{
        pub fn new(epochs: i32, alpha: f32) -> Self {
            Gradient{
                epochs: epochs,
                alpha: alpha,
            }
        }

        fn predict(x: i32, weight0: f32, weight1: f32) -> f32{
            return ((weight1*x)+weight0);
        }

        fn mse(x: Vec<i32>, y: Vec<i32>, weight0: f32, weight1: f32) -> f32{
            let mut cost: f32 = 0;
            
            for i in 0..x.len(){
                cost += f32::powf((self.predict(x[i], weight0, weight1) - y[i]),2.0);
            }
        }

        fn gradient_step(&self, weight0: f32, weight1: f32, x: Vec<i32>, y: Vec<i32>) -> (f32,f32){
            
            let mut error0: f32 = 0.0;
            let mut error1: f32 = 0.0;

            for i in 0..x.len(){
                erro0 += self.predict(x[i], weight0, weight1) - y[i];
                erro1 += (self.predict(x[i], weight0, weight1) - y[i]) * x[i];
            }

            let results: (f32, f32) = (
                alpha * (1/x.len()) * erro0,
                alpha * (1/x.len()) * erro1
            );
            
            return results;
        }

        pub fn gradient_descent(&self, weight0: f32, weight1: f32, x: Vec<i32>, y: Vec<i32>) -> (f32, f32, [f32;10]){
            let mut cost: [f32; &self.epochs] = [0; &self.epochs];

            for i in 0..&self.epochs{
                let new_values: (f32, f32) = self.gradient_step(weight0, weight1, x, y,
                                                                &self.alpha, 
                                                                &self.epochs);
                
                weight0 = new_values.0;
                weight1 = new_values.1;

                cost[i] = self.mse(x, y, weight0, weight1);

                println!("cost on {i}/{&self.epochs} : {}", cost[i]);
            }

            return (weight0, weight1, cost);
        }
    }
}