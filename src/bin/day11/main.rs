mod sim;

use std::env;
use std::fs;
use crate::sim::Simulator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let simulator = Simulator::new(&contents);

    part1(&simulator);
    part2(&simulator);
}

fn part1(sim: &Simulator) {
    let mut sim = sim.clone();
    sim.run_until_stable();

    println!("The number of occupied seats when stable is {}", sim.num_occupied())
}

fn part2(sim: &Simulator) {

}
