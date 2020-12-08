use nom::{IResult, Finish};
use nom::sequence::{preceded};
use nom::branch::alt;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{map_res, map, all_consuming};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::lib::std::collections::HashSet;

pub struct Console<'a> {
    insts: &'a Vec<Instruction>,
    pc: usize,
    acc: i32,
    already_executed: HashSet<usize>,
}

pub enum RunResult {
    Terminated,
    InfiniteLoop,
}

impl<'a> Console<'a> {
    pub fn new(insts: &'a Vec<Instruction>) -> Console<'a> {
        Console {
            insts,
            pc: 0,
            acc: 0,
            already_executed: HashSet::new(),
        }
    }

    pub fn step(&mut self) {
        self.already_executed.insert(self.pc);

        match &self.insts[self.pc] {
            Instruction::Nop(_) => self.pc += 1,
            Instruction::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            },
            Instruction::Jmp(offset) => {
                if *offset < 0 {
                    self.pc -= offset.abs() as usize
                } else {
                    self.pc += *offset as usize
                }
            }
        }
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            self.step();

            if self.already_executed.contains(&self.pc) {
                return RunResult::InfiniteLoop;
            }

            if self.pc == self.insts.len() {
                return RunResult::Terminated;
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

type ParseError<'a> = nom::error::Error<&'a str>;

impl Instruction {
    pub fn from_lines(s: &str) -> Result<Vec<Instruction>, ParseError> {
        all_consuming(instruction_list_parser)(s).finish().map(|(_, insts)| insts)
    }
}

fn parsed_digit1(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(s)
}

fn signed_number(s: &str) -> IResult<&str, i32> {
    alt((
        preceded(char('+'), parsed_digit1),
        preceded(char('-'), map(parsed_digit1, |num| -num)),
    ))(s)
}

fn instruction_parser(s: &str) -> IResult<&str, Instruction> {
    alt((
        map(preceded(tag("nop "), signed_number), Instruction::Nop),
        map(preceded(tag("acc "), signed_number), Instruction::Acc),
        map(preceded(tag("jmp "), signed_number), Instruction::Jmp),
    ))(s)
}

fn instruction_list_parser(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(newline, instruction_parser)(s)
}