use std::env;
use std::fs;
use std::num::ParseIntError;
use nom::lib::std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let mut nums = contents.lines().map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, ParseIntError>>()
        .expect("Could not parse lines as numbers");
    nums.sort();
    nums.push(nums[nums.len() - 1] + 3);
    nums.insert(0, 0);

    part1(&nums);
    part2(&nums);
}

fn part1(nums: &Vec<u32>) {
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for n in nums.iter().zip(&nums[1..]).map(|(a, b)| *b - *a) {
        counts.insert(n, counts.get(&n).cloned().unwrap_or(0) + 1);
    }

    let result = *counts.get(&1).unwrap() * *counts.get(&3).unwrap();
    println!("The product of the count of 1-diffs and 3-diffs is {}", result);
}

fn part2(nums: &Vec<u32>) {
    let mut cache = HashMap::new();
    println!("The adapters can be arranged {} different ways", count_arrangements(&nums[..], &mut cache));
}

fn count_arrangements(nums: &[u32], cache: &mut HashMap<u32, u64>) -> u64 {
    if nums.len() == 1 {
        return 1;
    }

    let curr_num = &nums[0];
    match cache.get(curr_num) {
        Some(total) => *total,
        _ => {
            let mut total = 0;

            let candidates = &nums[1..];
            for (i, n) in candidates.iter().enumerate() {
                if *n > curr_num + 3 {
                    break;
                }

                total += count_arrangements(&candidates[i..], cache);
            }

            cache.insert(*curr_num, total);
            total
        }
    }
}
