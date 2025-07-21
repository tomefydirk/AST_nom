
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
    fn result_binop_from(input:&str,left:Expr , right:Expr,operation:BinOp)->IResult<&str,Expr>{
       let result=(input,Expr::BinaryOp { left: (Box::new(left)), op : operation, right: Box::new(right) });
       IResult::Ok(result)
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

/*fn enlever_dernier_motif(input: &str, motif: &str) -> IResult<&str, &str> {
    if motif.is_empty() {
        return Ok((input, input)); // Rien à enlever
    }

    if let Some(pos) = input.rfind(motif) {
        let (avant, _) = input.split_at(pos);
        Ok(("", avant))
    } else {
        Ok(("", input)) // motif non trouvé → on renvoie l'input entier
    }
}*/


fn scantoken(input:&str) -> IResult<&str,&str>{

    alt((
        scan_digit,
        scan_plus,
        scan_moins
    )).parse(input)

    
}
fn parse_expr(input:&str)->IResult<&str,Expr>{
    let (reste,next_token)=scantoken(input)?;
    let a=parse_expr(input);
    
    if next_token=="-" || next_token=="+" || next_token=="*" || next_token=="/" {
            let (new_scaned,_)=scantoken(input)?; 
            let b=parse_term(new_scaned);
            return Expr::result_binop_from(new_scaned, a?.1,b?.1, BinOp::from_str(next_token));
    }
    Err(nom::Err::Error(Error::new(input, nom::error::ErrorKind::Digit)))

    
}
fn parse_factor(input:&str)->IResult<&str,Expr>{
    let (reste,next_token)=scantoken(input)?;
    let expr:IResult<&str,Expr>= match next_token.trim() {
        "("=>{
            let (new_reste,token)=scantoken(reste)?;
            let new_result=parse_expr(new_reste);
        /*    match new_result {
                Ok((current_expr,expr_value)) => {
                    if token == ")"{
                        return parse_expr(current_expr);
                    }else {
                        return   Err(nom::Err::Error(Error::new(input, nom::error::ErrorKind::Digit)))
                    }

                },
                Err(err) => {
                    return  Err(err);
                },
            } */
           
           todo!()
           
        },
        "-"=>{
            let (new_reste,_)=scantoken(reste)?;
            return  parse_factor(new_reste);
        },
        t => {
            let a=u32::from_str(t).map_err(|_|{
                nom::Err::Error(Error::new(t, nom::error::ErrorKind::Digit))
            })?;
            return IResult::Ok((t,Expr::Number(a)));
        } 
    };

}
fn parse_term(input:&str)->IResult<&str,Expr>{
    todo!()
}
fn main(){
    let a="(11() )";
    let v=parse_without_parens(a);
    println!("{v:?}");
}


