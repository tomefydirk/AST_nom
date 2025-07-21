
use std::{char, result, str::FromStr};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::opt, sequence::pair, IResult, Parser
};

#[derive(Debug)]
pub enum Expr {
    Number(u32),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Negate(Box<Expr>)
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
fn parse_expr(input:&str)->IResult<&str,Expr>{
    todo!()
}
fn parse_factor(input:&str)->IResult<&str,Expr>{
    let (reste,next_token)=scantoken(input)?;
    let expr:IResult<&str,Expr>= match next_token.trim() {
        "("=>{
            let new_reste=scantoken(reste);
            let new_expr=parse_expr(new_reste?.0);
            match new_expr {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
            todo!()
        },
        "-"=>{
            let (new_reste,_)=scantoken(reste)?;
            return  parse_factor(new_reste);
        }
        t => {
            let a=u32::from_str(t).map_err(|_|{
                nom::Err::Error(nom::error::Error::new(t, nom::error::ErrorKind::Digit))
            })?;
            return IResult::Ok((t,Expr::Number(a)));
        } 
    };
    todo!()
}
fn parse_term(){

}
fn main(){
    let a="312,2";
    let v=scantoken(a);
    println!("{v:?}");
}


