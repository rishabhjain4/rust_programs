use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() > 1 {

        let file_path = &args[1];
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening file: {}", e);
                return;
            }
        };
        let reader = BufReader::new(file);

        for line in reader.lines() {

            match line {
                Ok(line) => {
                    println!("{}", line);
                }
                Err(e) => {
                    println!("Error reading line: {}", e);
                }
            }
        }
    }
    else {
        println!("No arguments given");
    }

}
