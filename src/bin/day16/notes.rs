use nom::{IResult, Finish};
use std::ops::RangeInclusive;
use nom::combinator::{map_res, map, all_consuming};
use nom::character::complete::{char, digit1, newline};
use nom::sequence::separated_pair;
use nom::bytes::complete::{tag, is_not};
use nom::multi::separated_list1;

pub struct Notes {
    fields: Vec<Field>,
    your_ticket: Ticket,
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

    fn is_valid_value(&self, val: &u32) -> bool {
        self.fields.iter().any(|field| field.is_valid_value(val))
    }
}

pub struct Field {
    label: String,
    value_ranges: Vec<RangeInclusive<u32>>,
}

impl Field {
    fn is_valid_value(&self, val: &u32) -> bool {
        self.value_ranges.iter().any(|range| range.contains(val))
    }
}

pub struct Ticket(Vec<u32>);

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

