
use std::{char, result, str::FromStr};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt, sequence::pair, IResult, Parser
};

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
fn scan_plus(input:&str)-> IResult<&str,&str>{
    tag("+")(input)
}
fn scan_moins(input:&str)-> IResult<&str,&str>{
    tag("-")(input)
}
fn scan_digit(input:&str)-> IResult<&str,&str>{
    digit1(input)
}

fn scantoken(input:&str) -> IResult<&str,&str>{

    alt((
        scan_digit,
        scan_plus,
        scan_moins
    )).parse(input)

    
}
fn parse_expr(input:&str)->Expr{
    todo!()
}

fn main(){
    let a="312,2";
    let v=scantoken(a);
    println!("{v:?}");
}


