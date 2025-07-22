
use core::num;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::RSplit;
use std::{char, result, str::FromStr};

use nom::sequence::delimited;
use nom::Input;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::char,
    branch::alt, character::complete::digit1, combinator::opt, sequence::pair, Err, IResult, Parser
};
use nom::error::{Error, ParseError};

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
impl BinOp {
    fn from_str(a:&str)->Self{
        match a {
            "+"=>BinOp::Add,
            "-"=>BinOp::Sub,
            "*"=>BinOp::Mul,
            "/"=>BinOp::Div, 
           _=>BinOp::Add
        }
    }
}

impl Expr{
   fn result_binop_from(input: &str,left_box:Box<Expr>,right_box:Box<Expr>,operation:BinOp)->IResult<&str,Box<Expr>>{
       IResult::Ok((input,Box::new(Expr::BinaryOp { left: left_box , op: operation , right: right_box })))
   }
   fn some_box_binop_from(left_box:Box<Expr>,right_box:Box<Expr>,operation:BinOp)->Option<Box<Expr>>{
        Some(Box::new(Expr::BinaryOp { left: left_box , op: operation , right: right_box }))
   }
    fn result_number(input:&str,number:u32)->IResult<&str,Box<Expr>>{
        let result=(input,Box::new(Expr::Number(number)));
        IResult::Ok(result)       
    }
    fn result_from_current(input: &str,current_expr:Box<Expr>)->IResult<&str,Box<Expr>>{
        
        IResult::Ok((input,current_expr))
           
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

fn scan_plus(input:&str)-> IResult<&str,&str>{
    tag("+")(input)
}
fn scan_moins(input:&str)-> IResult<&str,&str>{
    tag("-")(input)
}
fn scan_digit(input:&str)-> IResult<&str,&str>{
    digit1(input)
}
fn parse_without_parens(input: &str) -> IResult<&str, &str> {
   let (input_1,_)=tag("(").parse(input)?; 
   let (input_farany,out_put)=take_while1(|c: char| c != ')').parse(input_1)?;
   let (f,_)=tag(")").parse(input_farany)?;
   IResult::Ok((f,out_put))
}




fn scantoken(input:&str) -> IResult<&str,&str>{

    alt((
        scan_digit,
        scan_plus,
        scan_moins
    )).parse(input.trim())

    
}

fn parse_expr(mut input: &str)->IResult<&str,Box<Expr>>{
    let mut next_token="";

    let mut current_expr:Option<Box<Expr>>=Option::Some(parse_term(Box::new(input))?.1);
    loop {
        if input.is_empty() {
            match current_expr {
                Some(result) =>{
                    return Expr::result_from_current(input, result);
                },
                None => {
                    return   Err(nom::Err::Error(Error::new(input, nom::error::ErrorKind::Digit)));
                }
            }
        }

        (input,next_token)=scantoken(input)?;
         if next_token=="+" || next_token=="-" {

           match current_expr {
            Some(left) => {
               
                current_expr=Expr::some_box_binop_from(left,parse_term(Box::new(input))?.1,BinOp::from_str(next_token));
              
            },
            None => {
                return   Err(nom::Err::Error(Error::new(input, nom::error::ErrorKind::Digit)));
            },
            
            }
        }
    }

}
fn parse_term(mut input:Box<&str>)->IResult<&str,Box<Expr>>{
    let (reste,next_token)=scantoken(*input)?;


    let n=u32::from_str(next_token).unwrap();

    let term_result=Expr::result_number(*input, n);
    
    if next_token.parse::<u32>().is_ok() {
        *input=reste;
        return term_result;
    }

    Err(nom::Err::Error(Error::new(*input, nom::error::ErrorKind::Digit)))
}
fn main(){
    let a="12   - 1  - 42";
    let v=parse_expr(a);
    let g: i32=v.unwrap().1.eval();
    println!("{:?}",g);
}


