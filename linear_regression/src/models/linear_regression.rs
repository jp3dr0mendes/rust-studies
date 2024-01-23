pub mod linear_regression{
    use std::fs;

    use self::gradient::Gradient;
    use self::dataset::Dataset;
    pub struct LinearRegression{
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
                cost: 0.0,
                data: Vec::new(),
                value: Vec::new(),
                gradient: Gradient::new(20, 0.5),
            }
        }

        pub fn train_model(mut self, epochs: i32, alpha: f32) {
            self.dataset.make_dataset();
            self.gradient = Gradient::new(epochs, alpha);

            let aux:(f32, f32, Vec<f32>) = self.gradient.gradient_descent(self.w0, self.w1,
                                                                          self.dataset.data, 
                                                                          self.dataset.value);

            self.w0   = aux.0;
            self.w1   = aux.1;
            self.cost = aux.2[9];

            fs::write(
                "model.txt",
                format!("w0: {}\nw1: {}", aux.0, aux.1)
            );
        }

        pub fn predict(&self, x: i32, y: i32) -> f32 {
            let x: f32 = x as f32;
            return ((self.w1*x) + &self.w0);
        }
    }

    mod gradient{
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
                let x: f32 = x as f32;
                return ((weight1*x)+weight0);
            }
    
            fn mse(&self, x: Vec<i32>, y: Vec<i32>, weight0: f32, weight1: f32) -> f32{
                let mut cost: f32 = 0.0;

                for i in 0..x.len(){
                    cost += f32::powf((Gradient::predict(x[i], weight0, weight1) - (y[i] as f32)),2.0);
                }

                return cost/(x.len() as f32);
                
            }
    
            fn gradient_step(&self, weight0: f32, weight1: f32, x: Vec<i32>, y: Vec<i32>) -> (f32,f32){
                
                let mut error0: f32 = 0.0;
                let mut error1: f32 = 0.0;
    
                for i in 0..x.len(){
                    error0 += Gradient::predict(x[i], weight0, weight1) - (y[i] as f32);
                    error1 += (Gradient::predict(x[i], weight0, weight1) - (y[i] as f32)) * (x[i] as f32);
                }
    
                let results: (f32, f32) = (
                    self.alpha * (1.0/(x.len() as f32)) * error0,
                    self.alpha * (1.0/(x.len() as f32)) * error1
                );
                
                return results;
            }
    
            pub fn gradient_descent(&mut self, weight0: f32, weight1: f32, x: Vec<i32>, y: Vec<i32>) -> (f32, f32, Vec<f32>){
                let mut costs: Vec<f32> = Vec::new();
                let mut weights: (f32, f32) = (0.0, 0.0);
    
                for i in 0..self.epochs{
                    weights = self.gradient_step(weight0, weight1, x.clone(), y.clone(),);
                    
                    // weight0 = new_values.0;
                    // weight1 = new_values.1;
                    let cost = self.mse(x.clone(), y.clone(), weights.0, weights.1);
                    // costs.push(self.mse(x, y, weights.0, weights.1));
                    costs.push(cost);
    
                    println!("cost on {i}/{} : {}", self.epochs, costs[i as usize]);
                }
    
                return (weight0, weight1, costs);
            }
        }
    }
    
    pub mod dataset{

        use std::fs::read_to_string;
        pub struct Dataset {
            itens: Vec<Vec<i32>>,
            pub data: Vec<i32>,
            pub value: Vec<i32>,
            path: String,
        }
    
        impl Dataset {
    
            pub fn new(path: &str) -> Self {
                Dataset {
                    itens: Vec::new(),        
                    path: path.to_string(),
                    data: Vec::new(),
                    value: Vec::new(),
                }
            }
    
            pub fn make_dataset(&mut self){
    
                for line in read_to_string(&self.path).unwrap().lines(){
                    
                    let line_content: Vec<String> = line.to_string().split(',').map(|s| s.to_string()).collect();
                    
                    if line_content != vec!["idade", "pressÃ"] {
                        self.itens.push(
                            vec![
                                line_content[0].parse::<i32>().expect("Error on item 1"), 
                                line_content[1].parse::<i32>().expect("Error on item 2")
                            ]
                        );
                    }
                }
    
                for i in 0..self.itens.len(){
                    self.data.push(self.itens[i][0]);
                    self.value.push(self.itens[i][1]);
                }
    
                println!("data: {:?} \n value: {:?}", &self.data, &self.value);
            }
        }
    }
}