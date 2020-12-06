mod day5;

use std::env;
use std::fs;
use crate::day5::{Seat, ParseError};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let seats = contents.lines()
        .map(|line| line.parse::<Seat>())
        .collect::<Result<Vec<Seat>, ParseError>>()
        .expect("Could not parse seats");

    part1(&seats);
}

fn part1(seats: &Vec<Seat>) {
    let highest_seat_id = seats.iter()
        .map(|s| s.id())
        .max()
        .unwrap();

    println!("The highest seat ID is {}", highest_seat_id);
}


