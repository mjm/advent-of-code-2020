use std::collections::HashMap;

use nom::{Finish, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, newline, satisfy};
use nom::combinator::{all_consuming, map, map_res};
use nom::error::Error;
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
                let matches = r.recognize(s, rule_num, &self.rules);
                matches.iter().any(|s| s.is_empty())
            }
        }
    }

    pub fn replace_rule(&mut self, rule_num: u32, s: &str) {
        let (_, r) = parse_rule(s).unwrap();
        self.rules.insert(rule_num, r);
    }
}

#[derive(Debug)]
enum Rule {
    Char(char),
    Subrules(Vec<u32>),
    Subrules2(Vec<u32>, Vec<u32>),
}

impl Rule {
    fn recognize<'a>(&self, s: &'a str, id: &u32, rules: &HashMap<u32, Rule>) -> Vec<&'a str> {
        use Rule::*;

        match self {
            Char(c) => {
                match char::<&str, Error<&str>>(*c)(s) {
                    Ok((s, _)) => vec![s],
                    Err(_) => vec![],
                }
            }
            Subrules(rule_ids) => {
                recognize_seq(&[s], id, rule_ids, rules)
            }
            Subrules2(rule_ids1, rule_ids2) => {
                let mut matches = recognize_seq(&[s], id, rule_ids1, rules);
                matches.append(&mut recognize_seq(&[s], id, rule_ids2, rules));
                matches
            }
        }
    }
}

fn recognize_seq<'a>(strs: &[&'a str], id: &u32, rule_ids: &[u32], rules: &HashMap<u32, Rule>) -> Vec<&'a str> {
    if strs.is_empty() || rule_ids.is_empty() {
        return strs.to_vec();
    }

    let mut res = Vec::new();
    let r = &rule_ids[0];
    let rule = rules.get(r).unwrap();

    for s in strs {
        let matches = rule.recognize(*s, r, rules);
        res.append(&mut recognize_seq(&matches[..], id, &rule_ids[1..], rules));
    }

    res
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
        },
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
