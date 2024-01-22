pub mod dataset{
    // use crate::models::dataset::File;
    use std::fs::read_to_string;
    pub struct Dataset {
        // itens: Vec<Vec<String>>,
        itens: Vec<Vec<i32>>,
        path: String,
    }

    impl Dataset {

        pub fn new(path: &str) -> Self {
            Dataset {
                itens: Vec::new(),
                path: path.to_string(),
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

            println!("{:?}", &self.itens);
        }
    }
}