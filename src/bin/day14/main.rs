mod ferry;

use std::env;
use std::fs;
use crate::ferry::{Instruction, Computer};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let insts = Instruction::from_lines(&contents).unwrap();

    part1(&insts);
}

fn part1(insts: &Vec<Instruction>) {
    let mut computer = Computer::new();
    computer.execute(insts);

    println!("The sum of all the non-zero addresses in memory is {}", computer.sum_memory());
}