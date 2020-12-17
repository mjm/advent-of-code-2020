mod notes;

use advent_of_code_2020::read_input;
use crate::notes::Notes;

fn main() {
    let contents = read_input();
    let notes = Notes::from(&contents).unwrap();

    part1(&notes);
    part2(&notes);
}

fn part1(notes: &Notes) {
    let invalid_values_total: u32 = notes.invalid_values().iter().map(|val| **val).sum();
    println!("The ticket scanning error rate is {}", invalid_values_total);
}

fn part2(notes: &Notes) {
    let fields = notes.ordered_fields();

    let mut result: u64 = 1;
    for (i, field) in fields.iter().enumerate() {
        if field.label.starts_with("departure ") {
            result *= notes.your_ticket.get(i) as u64;
        }
    }

    println!("The product of all the departure fields on my ticket is {}", result);
}