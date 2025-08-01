use nom::character::complete::space0;
use std::str::FromStr;
//use nom::combinator::opt;
use nom::error::Error;
use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1};
mod mathtools;
use mathtools::*;
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Negate(Box<Expr>),
    Ln(Box<Expr>),
    Sqrt(Box<Expr>),
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

    fn result_number(input: &str, number: f64) -> IResult<&str, Box<Expr>> {
        let result = (input, Box::new(Expr::Number(number)));
        IResult::Ok(result)
    }

    fn result_from_current(input: &str, current_expr: Box<Expr>) -> IResult<&str, Box<Expr>> {
        IResult::Ok((input, current_expr))
    }

    fn box_factorop_from(current_expr: Box<Expr>, token: &str) -> Box<Expr> {
        match token {
            "V" => Box::new(Expr::Sqrt(current_expr)),
            "ln" => Box::new(Expr::Ln(current_expr)),
            "-" => Box::new(Expr::Negate(current_expr)),
            a => {
                println!("operateur non trouvé :: {a}");
                Box::new(Expr::Negate(current_expr))
            }
        }
    }
}
impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::BinaryOp { left, op, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div => {
                        if r == 0.0 {
                            panic!("Division par zéro !");
                        }
                        l / r
                    }
                }
            }
            Expr::Negate(expr) => -expr.eval(),
            Expr::Ln(expr) => my_ln(expr.eval()),
            Expr::Sqrt(expr) => my_sqrt(expr.eval()),
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

/*----optional----*/

fn scan_ln(input: &str) -> IResult<&str, &str> {
    tag("ln")(input)
}
fn scan_sqrt(input: &str) -> IResult<&str, &str> {
    tag("V")(input)
}
/*
fn scan_float(input: &str) -> IResult<&str, f64> {

    let (rest, first_part) = digit1(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    if point.is_some() {
        let (rest3, second_part) = digit1(rest2)?;
        Ok((rest3, format!("{first_part}.{second_part}").parse().map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))? ))
    } else {
        Ok((rest, format!("{first_part}.0").parse().map_err(|_| nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))? ))
    }
}
*/

fn scantoken(input: &str) -> IResult<&str, &str> {
    alt((
        scan_ln, scan_sqrt, scan_digit, scan_plus, scan_moins, scan_div, scan_fois, parens0,
        parens1, space0,
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

    if next_token.parse::<f64>().is_ok() {
        let n = f64::from_str(next_token).unwrap();
        Expr::result_number(input, n)
    } else if next_token == "(" {
        return parse_real_factor(input);
    } else if next_token == "-" || next_token == "V" || next_token == "ln" {
        let perm = parse_factor(input);
        let (aff_perm, real_perm) = perm?;
        return IResult::Ok((aff_perm, Expr::box_factorop_from(real_perm, next_token)));
    } else {
        return Err(nom::Err::Error(Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }
}

fn main() {
    let a = "ln(12-7)";
    let v = parse_expr(a);

    println!("{:?}", v);
    let g: f64 = v.unwrap().1.eval();
    println!("{:?}", g);
}
