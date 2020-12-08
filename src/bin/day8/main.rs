mod console;

use std::env;
use std::fs;
use crate::console::{Instruction, Console};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let instructions = Instruction::from_lines(&contents)
        .unwrap();

    part1(&instructions);
}

fn part1(insts: &Vec<Instruction>) {
    let mut console = Console::new(insts);
    console.run_until_looped();
    println!("The value in the accumulator before looping is {}", console.acc());
}