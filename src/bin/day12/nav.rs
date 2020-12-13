use nom::{IResult, Finish};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{value, map_res, map, all_consuming};
use nom::character::complete::{digit1, newline};
use nom::sequence::tuple;
use nom::multi::separated_list1;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub enum Action {
    Direction(Direction),
    Left,
    Right,
    Forward,
}

pub struct Instruction(Action, i32);

impl Instruction {
    pub fn from_lines(s: &str) -> Result<Vec<Instruction>, nom::error::Error<&str>> {
        all_consuming(parse_instruction_lines)(s).finish().map(|(_, insts)| insts)
    }
}

fn parse_instruction_lines(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_instruction)(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    map(tuple((parse_action, parse_digit1)), |(a, n)| Instruction(a, n))(s)
}

fn parse_action(s: &str) -> IResult<&str, Action> {
    alt((
        value(Action::Direction(Direction::North), tag("N")),
        value(Action::Direction(Direction::East), tag("E")),
        value(Action::Direction(Direction::South), tag("S")),
        value(Action::Direction(Direction::West), tag("W")),
        value(Action::Left, tag("L")),
        value(Action::Right, tag("R")),
        value(Action::Forward, tag("F")),
    ))(s)
}

fn parse_digit1(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(s)
}

pub struct Navigator {
    pos: (i32, i32),
    dir: Direction,
}

impl Navigator {
    pub fn new() -> Self {
        Navigator {
            pos: (0, 0),
            dir: Direction::East,
        }
    }

    pub fn distance_from_origin(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs()
    }

    pub fn execute_all(&mut self, insts: &Vec<Instruction>) {
        for inst in insts {
            self.execute(inst);
        }
    }

    pub fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction(Action::Direction(dir), n) => {
                self.execute_move(*dir, *n);
            },
            Instruction(Action::Left, degrees) => {
                self.turn_left(*degrees);
            },
            Instruction(Action::Right, degrees) => {
                self.turn_right(*degrees);
            },
            Instruction(Action::Forward, n) => {
                self.execute_move(self.dir, *n);
            },
        }
    }

    fn execute_move(&mut self, dir: Direction, n: i32) {
        match dir {
            Direction::North => { self.pos = (self.pos.0, self.pos.1 + n); },
            Direction::East => { self.pos = (self.pos.0 + n, self.pos.1); },
            Direction::South => { self.pos = (self.pos.0, self.pos.1 - n); },
            Direction::West => { self.pos = (self.pos.0 - n, self.pos.1); },
        }
    }

    fn turn_left(&mut self, degrees: i32) {
        if degrees <= 0 {
            return;
        }

        match self.dir {
            Direction::North => { self.dir = Direction::West; },
            Direction::East => { self.dir = Direction::North; },
            Direction::South => { self.dir = Direction::East; },
            Direction::West => { self.dir = Direction::South; },
        }

        self.turn_left(degrees - 90);
    }

    fn turn_right(&mut self, degrees: i32) {
        if degrees <= 0 {
            return;
        }

        match self.dir {
            Direction::North => { self.dir = Direction::East; },
            Direction::East => { self.dir = Direction::South; },
            Direction::South => { self.dir = Direction::West; },
            Direction::West => { self.dir = Direction::North; },
        }

        self.turn_right(degrees - 90);
    }
}