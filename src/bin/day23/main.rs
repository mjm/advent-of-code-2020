use itertools::Itertools;

use crate::cups::Ring;

mod cups;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input: [usize; 9] = [4, 6, 3, 5, 2, 8, 1, 7, 9];
    let mut ring: Ring = (&input[..]).into();
    for _ in 0..100 {
        ring.perform_move();
    }

    let ordering = ring.cup_ordering();
    println!("The ordering of the cups is now {}", ordering);
}

fn part2() {
    let initial_nums: [usize; 9] = [4, 6, 3, 5, 2, 8, 1, 7, 9];
    let nums = (&initial_nums[..]).iter().cloned().chain(10..=1000000).collect_vec();

    let mut ring: Ring = (&nums[..]).into();
    for _ in 0..10000000 {
        ring.perform_move();
    }

    let (first, second) = ring.cups_after_one();
    println!("The cups with the stars under them are {} * {} = {}", first, second, first * second);
}