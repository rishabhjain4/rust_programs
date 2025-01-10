use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 1 {

        let file_path = &args[1];

        match fs::read_to_string(file_path) {
            Ok(contents) => {
                println!("File contents: {}", contents);
            }
            Err(e) => {
                println!("Error reading file: {}", e);
            }
        }
    }
    else {
        println!("No arguments given");
    }
}
