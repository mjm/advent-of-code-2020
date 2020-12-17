use nom::{IResult, Finish};
use std::ops::RangeInclusive;
use nom::combinator::{map_res, map, all_consuming};
use nom::character::complete::{char, digit1, newline};
use nom::sequence::separated_pair;
use nom::bytes::complete::{tag, is_not};
use nom::multi::separated_list1;
use std::iter::{once, FromIterator};
use std::collections::HashSet;

pub struct Notes {
    fields: Vec<Field>,
    pub your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Notes {
    pub fn from(s: &str) -> Result<Self, nom::error::Error<&str>> {
        all_consuming(
            map(
                separated_pair(
                    separated_pair(
                        separated_list1(newline, parse_field),
                        tag("\n\nyour ticket:\n"),
                        parse_ticket,
                    ),
                    tag("\n\nnearby tickets:\n"),
                    separated_list1(newline, parse_ticket),
                ),
                |((fields, your_ticket), nearby_tickets)| {
                    Notes { fields, your_ticket, nearby_tickets }
                },
            )
        )(s).finish().map(|(_, notes)| notes)
    }

    pub fn invalid_values(&self) -> Vec<&u32> {
        self.nearby_tickets.iter().flat_map(|Ticket(values)| {
            values.iter().filter(|val| !self.is_valid_value(*val))
        }).collect()
    }

    pub fn ordered_fields(&self) -> Vec<&Field> {
        let valid_tickets = self.valid_tickets();

        let mut candidates: Vec<HashSet<&Field>> = self.your_ticket.0.iter()
            .map(|_| HashSet::from_iter(self.fields.iter()))
            .collect();

        while !candidates.iter().all(|set| set.len() == 1) {
            for Ticket(vals) in &valid_tickets[..] {
                for (i, val) in vals.iter().enumerate() {
                    let valid_fields: HashSet<&Field> = candidates[i].iter().filter(|field| field.is_valid_value(val)).cloned().collect();
                    if valid_fields.len() < candidates[i].len() {
                        candidates[i] = valid_fields;
                        if candidates[i].len() == 1 {
                            self.eliminate(&mut candidates, i);
                        }
                    }
                }
            }
        }

        candidates.iter().map(|fields| *fields.iter().next().unwrap()).collect()
    }

    fn eliminate(&self, candidates: &mut Vec<HashSet<&Field>>, i: usize) {
        let field = *candidates[i].iter().next().unwrap();

        for j in 0..candidates.len() {
            if i == j {
                continue
            }

            if candidates[j].contains(field) {
                let mut fields = candidates[j].clone();
                fields.remove(field);
                candidates[j] = fields;
                if candidates[j].len() == 1 {
                    self.eliminate(candidates, j)
                }
            }
        }
    }

    fn is_valid_ticket(&self, ticket: &Ticket) -> bool {
        ticket.0.iter().all(|val| self.is_valid_value(val))
    }

    fn is_valid_value(&self, val: &u32) -> bool {
        self.fields.iter().any(|field| field.is_valid_value(val))
    }

    fn valid_tickets(&self) -> Vec<&Ticket> {
        once(&self.your_ticket).chain(&self.nearby_tickets)
            .filter(|ticket| self.is_valid_ticket(*ticket))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Field {
    pub label: String,
    value_ranges: Vec<RangeInclusive<u32>>,
}

impl Field {
    fn is_valid_value(&self, val: &u32) -> bool {
        self.value_ranges.iter().any(|range| range.contains(val))
    }
}

pub struct Ticket(Vec<u32>);

impl Ticket {
    pub fn get(&self, i: usize) -> u32 {
        self.0[i]
    }
}

fn parse_field(s: &str) -> IResult<&str, Field> {
    map(
        separated_pair(
            is_not(":"),
            tag(": "),
            parse_field_contents,
        ),
        |(label, value_ranges)| Field { label: String::from(label), value_ranges },
    )(s)
}

fn parse_field_contents(s: &str) -> IResult<&str, Vec<RangeInclusive<u32>>> {
    map(
        separated_pair(parse_range, tag(" or "), parse_range),
        |(first, second)| vec![first, second],
    )(s)
}

fn parse_ticket(s: &str) -> IResult<&str, Ticket> {
    map(separated_list1(char(','), parse_digit1), Ticket)(s)
}

fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(parse_digit1, char('-'), parse_digit1),
        |(start, end)| RangeInclusive::new(start, end),
    )(s)
}

fn parse_digit1(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(s)
}

