use crate::cups::Ring;

mod cups;

const INPUT: &str = "463528179";

fn main() {
    part1();
}

fn part1() {
    let mut ring = Ring::from(INPUT).unwrap();
    for _ in 0..100 {
        ring.perform_move();
    }

    let ordering = ring.cup_ordering();
    println!("The ordering of the cups is now {}", ordering);
}