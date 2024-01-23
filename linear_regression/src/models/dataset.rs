pub mod dataset{
    use std::fs::read_to_string;
    pub struct Dataset {
        itens: Vec<Vec<i32>>,
        data: Vec<i32>,
        value: Vec<i32>,
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