use advent_of_code_2020::read_input;

use crate::parse::{parse_input, RuleList};

mod parse;

fn main() {
    let contents = read_input();
    let (rules, input_strs) = parse_input(&contents).unwrap();

    part1(&rules, &input_strs);
}

fn part1(rules: &RuleList, input_strs: &Vec<&str>) {
    let match_count = input_strs.iter().filter(|s| rules.matches(&0, **s)).count();
    println!("The number of input strings matching rule 0 is {}", match_count);
}