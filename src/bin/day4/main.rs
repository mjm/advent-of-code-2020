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
    part2(&passports);
}

fn part1(passports: &Vec<Passport>) {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_count = passports.iter()
        .filter(|p| p.contains_fields(&required_fields))
        .count();

    println!("The number of passports with the required fields is {}", valid_count);
}

fn part2(passports: &Vec<Passport>) {
    let valid_count = passports.iter()
        .filter(|p| p.is_valid())
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

    fn is_valid(&self) -> bool {
        self.has_valid_birth_year() &&
            self.has_valid_issue_year() &&
            self.has_valid_exp_year() &&
            self.has_valid_height() &&
            self.has_valid_hair_color() &&
            self.has_valid_eye_color() &&
            self.has_valid_passport_id()
    }

    fn has_valid_birth_year(&self) -> bool {
        match self.fields.get("byr") {
            None => false,
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) if n >= 1920 && n <= 2002 => true,
                    _ => false,
                }
            }
        }
    }

    fn has_valid_issue_year(&self) -> bool {
        match self.fields.get("iyr") {
            None => false,
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) if n >= 2010 && n <= 2020 => true,
                    _ => false,
                }
            }
        }
    }

    fn has_valid_exp_year(&self) -> bool {
        match self.fields.get("eyr") {
            None => false,
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) if n >= 2020 && n <= 2030 => true,
                    _ => false,
                }
            }
        }
    }

    fn has_valid_height(&self) -> bool {
        match self.fields.get("hgt") {
            Some(s) if s.ends_with("cm") => {
                match s[..s.len() - 2].parse::<i32>() {
                    Ok(n) if n >= 150 && n <= 193 => true,
                    _ => false,
                }
            }
            Some(s) if s.ends_with("in") => {
                match s[..s.len() - 2].parse::<i32>() {
                    Ok(n) if n >= 59 && n <= 76 => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn has_valid_hair_color(&self) -> bool {
        match self.fields.get("hcl") {
            Some(s) if s.starts_with('#') => {
                s.chars().skip(1).all(|c| {
                    (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')
                })
            }
            _ => false,
        }
    }

    fn has_valid_eye_color(&self) -> bool {
        match self.fields.get("ecl") {
            Some(s) => *s == "amb" || *s == "blu" || *s == "brn" || *s == "gry" || *s == "grn" || *s == "hzl" || *s == "oth",
            _ => false,
        }
    }

    fn has_valid_passport_id(&self) -> bool {
        match self.fields.get("pid") {
            Some(s) => s.len() == 9 && s.chars().all(char::is_numeric),
            _ => false,
        }
    }
}