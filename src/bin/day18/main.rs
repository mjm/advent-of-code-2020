mod math;

use advent_of_code_2020::read_input;
use crate::math::Expr;

fn main() {
    let contents = read_input();
    let homework = Expr::parse_lines(&contents).unwrap();

    part1(&homework);
}

fn part1(homework: &Vec<Expr>) {
    let result: u64 = homework.iter().map(|expr| expr.evaluate()).sum();
    println!("The sum of the answer to each homework problem is {}", result);
}