
use std::{ str::FromStr};
use nom::character::complete::space0;
use nom::error::{Error};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag},
    character::complete::digit1,
};

#[derive(Debug)]
pub enum Expr {
    Number(u32),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Negate(Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
impl BinOp {
    fn from_str(a: &str) -> Self {
        match a {
            "+" => BinOp::Add,
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            _ => BinOp::Add,
        }
    }
}

impl Expr {
  
    fn box_binop_from(left_box: Box<Expr>, right_box: Box<Expr>, operation: BinOp) -> Box<Expr> {
        Box::new(Expr::BinaryOp {
            left: left_box,
            op: operation,
            right: right_box,
        })
    }
    fn result_number(input: &str, number: u32) -> IResult<&str, Box<Expr>> {
        let result = (input, Box::new(Expr::Number(number)));
        IResult::Ok(result)
    }
    fn result_from_current(input: &str, current_expr: Box<Expr>) -> IResult<&str, Box<Expr>> {
        IResult::Ok((input, current_expr))
    }
}
impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Number(n) => *n as i32,
            Expr::BinaryOp { left, op, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div => {
                        if r == 0 {
                            panic!("Division par zéro !");
                        }
                        l / r
                    }
                }
            }
            Expr::Negate(expr) => -expr.eval(),
        }
    }
}

fn scan_plus(input: &str) -> IResult<&str, &str> {
    tag("+")(input)
}
fn scan_moins(input: &str) -> IResult<&str, &str> {
    tag("-")(input)
}
fn scan_div(input: &str) -> IResult<&str, &str> {
    tag("/")(input)
}
fn scan_fois(input: &str) -> IResult<&str, &str> {
    tag("*")(input)
}
fn parens0(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}
fn parens1(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}

fn scan_digit(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn scantoken(input: &str) -> IResult<&str, &str> {
    alt((
        scan_digit, scan_plus, scan_moins, scan_div, scan_fois, parens0, parens1, space0,
    ))
    .parse(input.trim())
}

fn parse_expr(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token = "";
    let perm = parse_term(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    loop {
        if input.is_empty() {
            return Expr::result_from_current(input, current_expr);
        }
        if next_token == "+" || next_token == "-" {
            let scaned = parse_term(input)?;
            input = scaned.0;
            current_expr =
                Expr::box_binop_from(current_expr, scaned.1, BinOp::from_str(next_token));
        }

        (input, next_token) = scantoken(input)?;
    }
}

fn parse_term(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token;

    let perm = parse_factor(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    loop {
        let scaned = scantoken(input)?;
        if scaned.1 == "+" || scaned.1 == "-" || scaned.1 == ")" {
            return Expr::result_from_current(input, current_expr);
        } else {

            (input, next_token) = scaned;
        }

        if input.is_empty() {
            return Expr::result_from_current(input, current_expr);
        }
        if next_token == "*" || next_token == "/" {
            current_expr = Expr::box_binop_from(
                current_expr,
                parse_factor(input)?.1,
                BinOp::from_str(next_token),
            );
        }
    }
}
fn parse_real_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token = "";

    let perm = parse_term(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    loop {
        if next_token == ")" {
            return Expr::result_from_current(input, current_expr);
        }

        if next_token == "+" || next_token == "-" {
            let scaned = parse_term(input)?;
            input = scaned.0;
            current_expr =
                Expr::box_binop_from(current_expr, scaned.1, BinOp::from_str(next_token));
        }

        (input, next_token) = scantoken(input)?;
    }
}
fn parse_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let next_token;

    (input, next_token) = scantoken(input)?;

    if next_token.parse::<u32>().is_ok() {
        let n = u32::from_str(next_token).unwrap();
        Expr::result_number(input, n)
    } else if next_token == "(" {
        return parse_real_factor(input);
    } else if next_token == "-" {
        let perm = parse_factor(input);
        let (aff_perm, real_perm) = perm?;
        return IResult::Ok((aff_perm, Box::new(Expr::Negate(real_perm))));
    } else {
        return Err(nom::Err::Error(Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
}

fn main() {
    let a = "2*2*2+(2)";
    let v = parse_expr(a);

    println!("{:?}", v);
    let g: i32 = v.unwrap().1.eval();
    println!("{:?}", g);
}
