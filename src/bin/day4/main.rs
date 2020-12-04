use std::env;
use std::fs;
use std::collections::HashMap;
use std::convert::TryFrom;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let passports = contents.split("\n\n")
        .map(Passport::try_from)
        .collect::<Result<Vec<Passport>, ParseError>>()
        .expect("Could not parse list of passports");

    part1(&passports);
}

fn part1(passports: &Vec<Passport>) {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_count = passports.iter()
        .filter(|p| p.contains_fields(&required_fields))
        .count();

    println!("The number of valid passports is {}", valid_count);
}

#[derive(Debug, Clone)]
struct ParseError;

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let fields = s.split(|c| c == ' ' || c == '\n').map(|field_str| {
            let v = field_str.split(':').collect::<Vec<&str>>();
            if v.len() != 2 {
                Err(ParseError)
            } else {
                Ok((v[0], v[1]))
            }
        }).collect::<Result<HashMap<&str, &str>, ParseError>>()?;

        Ok(Passport { fields })
    }
}

impl<'a> Passport<'a> {
    fn contains_fields(&self, fields: &[&str]) -> bool {
        fields.iter().all(|field| self.fields.contains_key(field))
    }
}