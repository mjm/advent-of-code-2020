use nom::combinator::{all_consuming, map_res, map};
use nom::multi::{separated_list1, many1};
use nom::character::complete::{newline, digit1, char};
use nom::branch::alt;
use nom::{Finish, IResult};
use nom::sequence::{preceded, separated_pair};
use nom::bytes::complete::tag;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Mask(Vec<char>),
    Mem(u64, u64),
}

impl Instruction {
    pub fn from_lines(s: &str) -> Result<Vec<Instruction>, nom::error::Error<&str>> {
        all_consuming(separated_list1(newline, alt((
            parse_mask_inst,
            parse_mem_inst,
        ))))(s).finish().map(|(_, insts)| insts)
    }
}

fn parse_mask_inst(s: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mask = "), many1(alt((char('X'), char('0'), char('1'))))), Instruction::Mask)(s)
}

fn parse_mem_inst(s: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mem["), separated_pair(parse_digit1, tag("] = "), parse_digit1)), |(addr, value)| {
        Instruction::Mem(addr, value)
    })(s)
}

fn parse_digit1(s: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(s)
}

pub struct Computer {
    mask: Vec<char>,
    mem: HashMap<u64, u64>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            mask: Vec::new(),
            mem: HashMap::new(),
        }
    }

    pub fn step(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Mask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::Mem(addr, value) => {
                self.mem.insert(*addr, self.mask(value));
            }
        }
    }

    pub fn execute(&mut self, insts: &Vec<Instruction>) {
        for inst in insts {
            self.step(inst);
        }
    }

    pub fn sum_memory(&self) -> u64 {
        self.mem.values().sum()
    }

    fn mask(&self, value: &u64) -> u64 {
        let mut new_value = *value;
        for (i, c) in self.mask.iter().rev().enumerate() {
            new_value = match c {
                '0' => new_value & !(1 << i),
                '1' => new_value | (1 << i),
                _ => new_value,
            }
        }
        new_value
    }
}