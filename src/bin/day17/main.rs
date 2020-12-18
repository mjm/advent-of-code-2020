mod sim;

use advent_of_code_2020::read_input;
use crate::sim::Simulation;

fn main() {
    let contents = read_input();

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let mut sim = Simulation::from_3d(contents);
    sim.simulate(6);
    println!("The number of active points after 6 cycles is {}", sim.num_active_points());
}

fn part2(contents: &str) {
    let mut sim = Simulation::from_4d(contents);
    sim.simulate(6);
    println!("The number of active points after 6 cycles in 4D is {}", sim.num_active_points());
}