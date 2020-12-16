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
}

fn part1(problem: &Problem) {
    let (next_bus_id, time_to_arrive) = problem.next_bus();
    println!("The next bus is {}, arriving in {} minutes, so the answer is {}", next_bus_id, time_to_arrive, next_bus_id * time_to_arrive);
}

#[derive(Debug)]
struct Problem {
    leave_estimate: u32,
    buses: Vec<u32>,
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
                Problem { leave_estimate, buses: buses.iter().flat_map(|x| *x).collect() }
            })
        )(s).finish().map(|(_, problem)| problem)
    }

    fn next_bus(&self) -> (u32, u32) {
        self.buses.iter().map(|bus| {
            (*bus, *bus - (self.leave_estimate % *bus))
        }).min_by_key(|(_, time_to_arrive)| *time_to_arrive).unwrap()
    }
}

fn parse_digit1(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(s)
}

