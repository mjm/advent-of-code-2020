mod nav;

use std::env;
use std::fs;
use crate::nav::{Instruction, Navigator, WaypointNavigator};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let insts = Instruction::from_lines(&contents).unwrap();

    part1(&insts);
    part2(&insts);
}

fn part1(insts: &Vec<Instruction>) {
    let mut nav = Navigator::new();
    nav.execute_all(insts);

    println!("The Manhattan distance of the ship from the start is {}", nav.distance_from_origin());
}

fn part2(insts: &Vec<Instruction>) {
    let mut nav = WaypointNavigator::new();
    nav.execute_all(insts);

    println!("The Manhattan distance of the ship from the start is {}", nav.distance_from_origin());
}