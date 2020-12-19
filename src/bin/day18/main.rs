mod math;

use advent_of_code_2020::read_input;
use crate::math::Expr;

fn main() {
    let contents = read_input();

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &str) {
    let homework = Expr::parse_lines(contents).unwrap();
    let result: u64 = homework.iter().map(|expr| expr.evaluate()).sum();
    println!("The sum of the answer to each homework problem is {}", result);
}

fn part2(contents: &str) {
    let homework = Expr::parse_lines_ordered(contents).unwrap();
    let result: u64 = homework.iter().map(|expr| expr.evaluate()).sum();
    println!("The sum of the answer to each homework problem with precedence is {}", result);
}