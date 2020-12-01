use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");

    let nums = to_numbers(&contents);

    for (i, num) in nums.iter().enumerate() {
        for (j, num2) in nums.iter().enumerate() {
            if i != j && num + num2 == 2020 {
                println!("The product of {} and {} is {}", num, num2, num * num2);
                return
            }
        }
    }
}

fn to_numbers(contents: &str) -> Vec<i32> {
    contents.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}