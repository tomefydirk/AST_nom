use nom::{
    IResult,
    character::complete::digit1,
    combinator::map_res,
};
#[derive(Debug)]
pub enum Expr {
    Number(i64),
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

fn main(){

}