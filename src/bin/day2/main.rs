use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let passwords = contents.lines()
        .map(|line| PolicyPasswordPair::from(line))
        .collect::<Option<Vec<PolicyPasswordPair>>>()
        .expect("Some line is not parsing correctly");

    part1(&passwords)
}

fn part1(passwords: &Vec<PolicyPasswordPair>) {
    let valid_password_count = passwords.iter()
        .filter(|PolicyPasswordPair(policy, password)| {
            let char_count = password.chars().filter(|c| *c == policy.letter).count();
            char_count <= policy.max_times as usize && char_count >= policy.min_times as usize
        })
        .count();

    println!("There are {} passwords that match their policy.", valid_password_count);
}

#[derive(Debug)]
struct PolicyPasswordPair<'a>(PasswordPolicy, &'a str);

impl PolicyPasswordPair<'_> {
    fn from<'a>(s: &'a str) -> Option<PolicyPasswordPair<'a>> {
        let v = s.split(": ").collect::<Vec<&str>>();
        match &v[..] {
            [policy, password] => {
                let parsed_policy = PasswordPolicy::from(policy)?;
                Some(PolicyPasswordPair(parsed_policy, password))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    min_times: u32,
    max_times: u32,
}

impl PasswordPolicy {
    fn from(s: &str) -> Option<PasswordPolicy> {
        let v = s.split(' ').collect::<Vec<&str>>();
        match &v[..] {
            [times, letter_str] => {
                let letter = letter_str.chars().next()?;

                let v2 = times.split('-').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
                match &v2[..] {
                    [min_times, max_times] => Some(PasswordPolicy {
                        letter,
                        min_times: *min_times,
                        max_times: *max_times,
                    }),
                    _ => None,
                }
            }
            _ => None
        }
    }
}