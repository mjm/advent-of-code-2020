use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, satisfy, alpha1};
use nom::combinator::{map, map_res, all_consuming};
use nom::{IResult, Finish};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};

pub struct RuleList {
    rules: HashMap<u32, Rule>,
}

impl RuleList {
    pub fn matches(&self, rule_num: &u32, s: &str) -> bool {
        match self.rules.get(rule_num) {
            None => false,
            Some(r) => {
                match r.recognize(s, &self.rules) {
                    Some(s) if s.is_empty() => true,
                    _ => false,
                }
            }
        }
    }
}

enum Rule {
    Char(char),
    Subrules(Vec<u32>),
    Subrules2(Vec<u32>, Vec<u32>),
}

impl Rule {
    fn recognize<'a>(&self, s: &'a str, rules: &HashMap<u32, Rule>) -> Option<&'a str> {
        use Rule::*;

        if s.is_empty() {
            return None;
        }

        match self {
            Char(c) => {
                if s.starts_with(*c) {
                    Some(&s[1..])
                } else {
                    None
                }
            }
            Subrules(rule_ids) => {
                recognize_seq(s, rule_ids, rules)
            }
            Subrules2(rule_ids1, rule_ids2) => {
                recognize_seq(s, rule_ids1, rules).or_else(|| recognize_seq(s, rule_ids2, rules))
            }
        }
    }
}

fn recognize_seq<'a>(s: &'a str, rule_ids: &Vec<u32>, rules: &HashMap<u32, Rule>) -> Option<&'a str> {
    let mut s = s;

    for r in rule_ids {
        let rule = rules.get(r)?;
        match rule.recognize(s, rules) {
            Some(s1) => {
                s = s1;
            }
            None => {
                return None;
            }
        };
    }

    Some(s)
}

pub fn parse_input(s: &str) -> Result<(RuleList, Vec<&str>), nom::error::Error<&str>> {
    all_consuming(
        separated_pair(parse_rule_list, tag("\n\n"), parse_input_strings)
    )(s).finish().map(|(_, out)| out)
}

fn parse_rule_list(s: &str) -> IResult<&str, RuleList> {
    map(
        separated_list1(newline, parse_numbered_rule),
        |rules| {
            let mut rules_by_num = HashMap::new();
            for (num, rule) in rules {
                rules_by_num.insert(num, rule);
            }
            RuleList { rules: rules_by_num }
        }
    )(s)
}

fn parse_input_strings(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(newline, alpha1)(s)
}

fn parse_numbered_rule(s: &str) -> IResult<&str, (u32, Rule)> {
    separated_pair(parse_digit1, tag(": "), parse_rule)(s)
}

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    alt((
        map(
            preceded(char('"'), terminated(satisfy(|c| c.is_alphabetic()), char('"'))),
            Rule::Char,
        ),
        map(
            separated_pair(
                separated_list1(char(' '), parse_digit1),
                tag(" | "),
                separated_list1(char(' '), parse_digit1)),
            |(l1, l2)| Rule::Subrules2(l1, l2),
        ),
        map(
            separated_list1(char(' '), parse_digit1),
            Rule::Subrules,
        ),
    ))(s)
}

fn parse_digit1(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(s)
}
