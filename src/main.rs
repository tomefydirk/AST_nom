
use std::str::FromStr;

use nom::{
    character::complete::{digit1, space0}, combinator::map_res, number, sequence::delimited, IResult
};
use regex::Regex;


#[derive(Debug)]
pub enum Expr {
    Number(Number),
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

#[derive(Debug)]
pub enum Number{
    Pos(f64),
    Neg(f64)
}

pub  trait Evaluable {
    fn eval(&self) -> f64 ;   
}
impl  Evaluable for Number {
    fn eval(&self) -> f64  {
        match self {
            Number::Pos(n) => *n ,
            Number::Neg(n) => -(*n),
        }
    }
}
impl Evaluable for Expr {
    fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => n.eval(),
            Expr::BinaryOp { left, op, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div => l / r,
                }
            }
        }
    }
}

fn premier_nombre(phrase: &str) -> Option<f64> {
    let re = Regex::new(r"-?\d+(\.\d+)?").unwrap();
    if let Some(mat) = re.find(phrase) {
        mat.as_str().parse::<f64>().ok()
    } else {
        None
    }
}

fn main() {
    let texte = "La température est descendue à -1212.5 degrés hier soir.";
    match premier_nombre(texte) {
        Some(n) => println!("Premier nombre trouvé : {}", n),
        None => println!("Aucun nombre trouvé."),
    }
}