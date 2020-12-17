mod notes;

use advent_of_code_2020::read_input;
use crate::notes::Notes;

fn main() {
    let contents = read_input();
    let notes = Notes::from(&contents).unwrap();

    part1(&notes);
}

fn part1(notes: &Notes) {
    let invalid_values_total: u32 = notes.invalid_values().iter().map(|val| **val).sum();
    println!("The ticket scanning error rate is {}", invalid_values_total);
}