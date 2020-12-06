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
    part2(&seats);
}

fn part1(seats: &Vec<Seat>) {
    let highest_seat_id = seats.iter()
        .map(|s| s.id())
        .max()
        .unwrap();

    println!("The highest seat ID is {}", highest_seat_id);
}

fn part2(seats: &Vec<Seat>) {
    let mut seat_ids = seats.iter()
        .map(Seat::id)
        .collect::<Vec<i32>>();
    seat_ids.sort();

    for i in 0..(seat_ids.len() - 1) {
        let this_seat = seat_ids[i];
        let next_seat =  seat_ids[i + 1];
        if this_seat + 1 != next_seat {
            println!("Your seat ID is {}", this_seat + 1);
            return;
        }
    }

    panic!("Didn't find a gap in the list of seats!")
}

