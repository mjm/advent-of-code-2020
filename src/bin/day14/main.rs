mod ferry;

use crate::ferry::{Instruction, Computer};
use advent_of_code_2020::read_input;

fn main() {
    let contents = read_input();
    let insts = Instruction::from_lines(&contents).unwrap();

    part1(&insts);
    part2(&insts);
}

fn part1(insts: &Vec<Instruction>) {
    let mut computer = Computer::new();
    computer.execute(insts);

    println!("The sum of all the non-zero addresses in memory is {}", computer.sum_memory());
}

fn part2(insts: &Vec<Instruction>) {
    let mut computer = Computer::new();
    computer.execute_v2(insts);

    println!("The sum of all the non-zero addresses in memory (v2) is {}", computer.sum_memory());
}