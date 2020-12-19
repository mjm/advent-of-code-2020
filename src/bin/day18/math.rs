use nom::{Err, Finish, IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::{all_consuming, map, map_res, value};
use nom::error::{ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Mult,
}

#[derive(Debug)]
pub enum Expr {
    Num(u64),
    Op(Operator, Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

impl Expr {
    pub fn parse_lines(s: &str) -> Result<Vec<Expr>, nom::error::Error<&str>> {
        all_consuming(
            separated_list1(newline, parse_expr_chain),
        )(s).finish().map(|(_, exprs)| exprs)
    }

    pub fn parse_lines_ordered(s: &str) -> Result<Vec<Expr>, nom::error::Error<&str>> {
        all_consuming(
            separated_list1(newline, parse_expr_chain_ordered),
        )(s).finish().map(|(_, exprs)| exprs)
    }

    pub fn evaluate(&self) -> u64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Op(Operator::Add, lhs, rhs) => {
                lhs.evaluate() + rhs.evaluate()
            }
            Expr::Op(Operator::Mult, lhs, rhs) => {
                lhs.evaluate() * rhs.evaluate()
            },
            Expr::Paren(expr) => expr.evaluate(),
        }
    }
}

fn parse_num(s: &str) -> IResult<&str, Expr> {
    map(parse_digit1, Expr::Num)(s)
}

fn parse_paren_expr(s: &str) -> IResult<&str, Expr> {
    map(
        preceded(char('('), terminated(parse_expr_chain, char(')'))),
        |expr| Expr::Paren(Box::new(expr)),
    )(s)
}

fn parse_expr_chain(mut s: &str) -> IResult<&str, Expr> {
    let mut elem = alt((
        parse_num,
        parse_paren_expr,
    ));

    let (s1, o) = elem.parse(s)?;
    let mut res = o;
    s = s1;

    loop {
        match parse_operator.parse(s) {
            Err(Err::Error(_)) => return Ok((s, res)),
            Err(e) => return Err(e),
            Ok((s1, op)) => {
                if s1 == s {
                    return Err(Err::Error(nom::error::Error::from_error_kind(s1, ErrorKind::SeparatedList)));
                }

                match elem.parse(s1) {
                    Err(Err::Error(_)) => return Ok((s, res)),
                    Err(e) => return Err(e),
                    Ok((s2, o)) => {
                        res = Expr::Op(op, Box::new(res), Box::new(o));
                        s = s2;
                    }
                }
            }
        }
    }
}

fn parse_paren_expr_ordered(s: &str) -> IResult<&str, Expr> {
    map(
        preceded(char('('), terminated(parse_expr_chain_ordered, char(')'))),
        |expr| Expr::Paren(Box::new(expr)),
    )(s)
}

fn parse_expr_chain_ordered(mut s: &str) -> IResult<&str, Expr> {
    let mut elem = alt((
        parse_num,
        parse_paren_expr_ordered,
    ));

    let (s1, o) = elem.parse(s)?;
    let mut res = o;
    s = s1;

    loop {
        match parse_operator.parse(s) {
            Err(Err::Error(_)) => return Ok((s, res)),
            Err(e) => return Err(e),
            Ok((s1, op)) => {
                if s1 == s {
                    return Err(Err::Error(nom::error::Error::from_error_kind(s1, ErrorKind::SeparatedList)));
                }

                match elem.parse(s1) {
                    Err(Err::Error(_)) => return Ok((s, res)),
                    Err(e) => return Err(e),
                    Ok((s2, o)) => {
                        res = match (&op, res) {
                            (Operator::Add, Expr::Op(op2, lhs, rhs)) => {
                                Expr::Op(op2, lhs, Box::new(Expr::Op(op, rhs, Box::new(o))))
                            },
                            (_, res) => Expr::Op(op, Box::new(res), Box::new(o)),
                        };
                        s = s2;
                    }
                }
            }
        }
    }
}

fn parse_operator(s: &str) -> IResult<&str, Operator> {
    alt((
        value(Operator::Add, tag(" + ")),
        value(Operator::Mult, tag(" * ")),
    ))(s)
}

fn parse_digit1(s: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(s)
}

