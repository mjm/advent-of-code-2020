use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");

    let nums = to_numbers(&contents);

    part1(&nums);
    part2(&nums);
}

fn part1(nums: &Vec<i32>) {
    for (i, num) in nums.iter().enumerate() {
        for num2 in &nums[i + 1..] {
            if num + num2 == 2020 {
                println!("The product of {} and {} is {}", num, num2, num * num2);
                return;
            }
        }
    }
}

fn part2(nums: &Vec<i32>) {
    for (i, num) in nums.iter().enumerate() {
        for (j, num2) in (&nums[i + 1..]).iter().enumerate() {
            for num3 in &nums[j + 1..] {
                if num + num2 + num3 == 2020 {
                    println!("The product of {}, {}, and {} is {}", num, num2, num3, num * num2 * num3);
                    return;
                }
            }
        }
    }
}

fn to_numbers(contents: &str) -> Vec<i32> {
    contents.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}