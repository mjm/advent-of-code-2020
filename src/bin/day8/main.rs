mod console;

use std::env;
use std::fs;
use crate::console::{Instruction, Console, RunResult};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let instructions = Instruction::from_lines(&contents)
        .unwrap();

    part1(&instructions);
    part2(&instructions);
}

fn part1(insts: &Vec<Instruction>) {
    let mut console = Console::new(insts);
    console.run();
    println!("The value in the accumulator before looping is {}", console.acc());
}

fn part2(insts: &Vec<Instruction>) {
    let (i, acc) = (0..insts.len())
        .filter(|i| {
            match &insts[*i] {
                Instruction::Acc(_) => false,
                _ => true,
            }
        })
        .map(|i| {
            let mut new_insts = insts.clone();
            new_insts[i] = match &insts[i] {
                Instruction::Nop(n) => Instruction::Jmp(*n),
                Instruction::Jmp(n) => Instruction::Nop(*n),
                _ => panic!("Should not have reached this if the instruction wasn't a jmp or nop"),
            };
            (i, new_insts)
        })
        .find_map(|(i, insts)| {
            let mut console = Console::new(&insts);
            match console.run() {
                RunResult::Terminated => Some((i, console.acc())),
                RunResult::InfiniteLoop => None,
            }
        })
        .expect("Did not find a change in the instructions that resulted in termination");

    println!("Swapping the instruction at index {} allowed the program to terminate with accumulator value {}", i, acc);
}