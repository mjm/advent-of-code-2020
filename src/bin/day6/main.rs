use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let groups = parse_groups(&contents);

    part1(&groups);
    part2(&groups);
}

fn part1(groups: &Vec<HashSet<char>>) {
    let result: usize = groups.iter().map(HashSet::len).sum();
    println!("The sum of counts of answered questions is {}", result);
}

fn part2(groups: &Vec<HashSet<char>>) {

}

fn parse_groups(contents: &str) -> Vec<HashSet<char>> {
    contents.split("\n\n").map(|group_str| {
        group_str.chars().filter(|c| *c >= 'a' && *c <= 'z').collect::<HashSet<char>>()
    }).collect()
}