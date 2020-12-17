mod sim;

use advent_of_code_2020::read_input;
use crate::sim::Simulation;

fn main() {
    let contents = read_input();

    part1(&contents);
}

fn part1(contents: &str) {
    let mut sim = Simulation::from(contents);
    sim.simulate(6);
    println!("The number of active points after 6 cycles is {}", sim.num_active_points());
}