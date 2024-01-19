pub mod dataset{
    // use crate::models::dataset::File;
    use std::fs::File;
    use std::io::{Read, BufReader};
    pub struct Dataset {
        itens: Vec<(i32,i32)>,
        path: String,
    }

    impl Dataset{

        pub fn new(path: &str) -> Self {
            Dataset {
                itens: Vec::new(),
                path: path.to_string(),
            }
        }

        pub fn make_dataset(&self){
            let file = File::open(self.path)?;
            let reader = BufReader::new(file);

            for line in reader.lines(){
                println!("{}", line);
            }
        }
    }
}