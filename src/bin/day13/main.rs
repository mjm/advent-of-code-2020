use std::env;
use std::fs;
use nom::combinator::{map_res, all_consuming, map, value};
use nom::{IResult, Finish};
use nom::character::complete::{digit1, newline};
use nom::sequence::separated_pair;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::branch::alt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let problem = Problem::from_str(&contents).unwrap();

    part1(&problem);
    part2(&problem);
}

fn part1(problem: &Problem) {
    let (next_bus_id, time_to_arrive) = problem.next_bus();
    println!("The next bus is {}, arriving in {} minutes, so the answer is {}", next_bus_id, time_to_arrive, next_bus_id * time_to_arrive);
}

fn part2(problem: &Problem) {
    println!("The time when all the buses start lining up correctly is minute {}", problem.contest_winning_time());
}

#[derive(Debug)]
struct Problem {
    leave_estimate: u64,
    buses: Vec<Option<u64>>,
}

impl Problem {
    fn from_str(s: &str) -> Result<Problem, nom::error::Error<&str>> {
        all_consuming(
            map(separated_pair(
                parse_digit1,
                newline,
                separated_list1(tag(","), alt((
                    map(parse_digit1, Some),
                    value(None, tag("x"))
                )))
            ), |(leave_estimate, buses)| {
                Problem { leave_estimate, buses }
            })
        )(s).finish().map(|(_, problem)| problem)
    }

    fn next_bus(&self) -> (u64, u64) {
        self.buses.iter().flat_map(|bus| {
            let bus = (*bus)?;
            Some((bus, bus - (self.leave_estimate % bus)))
        }).min_by_key(|(_, time_to_arrive)| *time_to_arrive).unwrap()
    }
    
    fn contest_winning_time(&self) -> u64 {
        let pairs: Vec<(u64, u64)> = self.buses.iter().enumerate().flat_map(|(i, id)| {
            let id = (*id)?;

            let a = id - ((i as u64) % id);
            Some((if a == id { 0 } else { a }, id))
        }).collect();

        let big_m: u64 = pairs.iter().map(|(_, id)| *id).product();
        pairs.iter().fold(0 as u64, |total, (a, id)| {
            let b = big_m / id;
            let s = mod_inv(b, *id);

            (total + (((a * b) % big_m) * s) % big_m) % big_m
        })
    }
}

fn parse_digit1(s: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(s)
}

// compute the modular multiplicative inverse of a mod n using extended Euclidean algorithm
fn mod_inv(a: u64, n: u64) -> u64 {
    let (mut old_r, mut r) = (a, n);
    let (mut old_s, mut s): (i64, i64) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;

        let new_r = old_r - quotient * r;
        old_r = r;
        r = new_r;

        let new_s = old_s - (quotient as i64) * s;
        old_s = s;
        s = new_s;
    }

    if old_s < 0 {
        n - (old_s.abs() as u64)
    } else {
        old_s as u64
    }
}