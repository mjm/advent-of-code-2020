use std::env;
use std::fs;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let nums = contents.lines().map(|s| s.parse::<u64>()).collect::<Result<Vec<u64>, ParseIntError>>()
        .expect("Could not parse lines as numbers");

    part1(&nums);
}

fn part1(nums: &Vec<u64>) {
    for i in 25..nums.len() {
        let num = &nums[i];
        let previous_nums = &nums[i-25..i];

        if !sums_of_pairs(previous_nums).iter().any(|n| n == num) {
            println!("The number {} (idx {}) does not equal the sum of two of the 25 preceding numbers.", num, i);
            return;
        }
    }

    panic!("Oh no! We didn't find a number that matched our conditions");
}

fn sums_of_pairs(nums: &[u64]) -> Vec<u64> {
    (&nums[..nums.len()-1]).iter().enumerate().flat_map(|(i, m)| {
        let m = *m;
        (&nums[i+1..nums.len()]).iter().map(move |n| *n + m)
    }).collect()
}