use nom::IResult;
use nom::error::Error;
use crate::expr_struct::{BinOp, Expr};
use crate::tokentool::{Token, scan_token};
/* 
rhs:Right hand side
lhs:Left hand side
*/
fn parse_binop_level<'a, F>(
    input: &'a str,
    lower_parser: F,
    ops: &[&str],
) -> IResult<&'a str, Box<Expr>>
where
    F: Fn(&'a str) -> IResult<&'a str, Box<Expr>>,
{
    let (mut input_rem, mut current_expr) = lower_parser(input)?;

    if input_rem.starts_with(')') {
        return Expr::result_from_current(input_rem, current_expr);
    }

    loop {
        let (next_input, token) = scan_token(input_rem)?;

        match token {
            Token::Other(op) if ops.contains(&op) => {
                let (after_rhs, rhs) = lower_parser(next_input)?;
                current_expr = Expr::box_binop_from(current_expr, rhs, BinOp::from_str(op));
                input_rem = after_rhs;
            }

            //pour les cas implicite de multiplication (2)2 ou 2(2)
            Token::Number(n) => {

                if ops.contains(&"*") {
                    current_expr = Expr::box_binop_from(
                        current_expr,
                        Box::new(Expr::Number(n)),
                        BinOp::Mul,
                    );
                    input_rem = next_input;
                } else {
                    return Expr::result_from_current(input_rem, current_expr);
                }
            }
            Token::Other("(") if ops.contains(&"*") => {
            
                let (after_rhs, rhs) = lower_parser(next_input)?;
                current_expr = Expr::box_binop_from(current_expr, rhs, BinOp::Mul);
                input_rem = after_rhs;
            }
            _ => return Expr::result_from_current(input_rem, current_expr),
        }
    }
}


pub fn parse_expr(input: &str) -> IResult<&str, Box<Expr>> {
    parse_binop_level(input, parse_term, &["+", "-"])
}


pub fn parse_term(input: &str) -> IResult<&str, Box<Expr>> {
    parse_binop_level(input, parse_power, &["*", "/"])
}

pub fn parse_power(input: &str) -> IResult<&str, Box<Expr>> {
    parse_binop_level(input, parse_factor, &["^"])
}

pub fn parse_factor( input: &str) -> IResult<&str, Box<Expr>> {
    let (next_input, token) = scan_token(input)?;

    match token {
        Token::Number(n) => Expr::result_number(next_input, n),
        Token::Other(str_token) => {
            if str_token == "(" {
                parse_real_factor(next_input)
            } else if Expr::is_factor_op(str_token) {
                let (after, real_perm) = parse_factor(next_input)?;
                Ok((after, Expr::box_factorop_from(real_perm, str_token)))
            } else {
                Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )))
            }
        }
    }
}

pub fn parse_real_factor(input: &str) -> IResult<&str, Box<Expr>> {
    let (after_expr, expr) = parse_expr(input)?;
    let (after_paren, token) = scan_token(after_expr)?;

    match token {
        Token::Other(")") => Expr::result_from_current(after_paren, expr),
        _ => Err(nom::Err::Error(Error::new(
            after_paren,
            nom::error::ErrorKind::Digit,
        ))),
    }
}
