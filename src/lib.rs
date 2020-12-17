use std::env;
use std::fs;

pub fn read_input() -> String {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    fs::read_to_string(input_path)
        .expect("Something went wrong reading the file")
}