use nom::{
    character::complete::digit1, combinator::map_res, number, IResult
};
#[derive(Debug)]
pub enum Expr {
    Number(Number)
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

fn parser(expr_str:&String) -> Expr {
    todo!()
}
fn main(){

}