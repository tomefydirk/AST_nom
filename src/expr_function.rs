use nom::IResult;
use nom::error::Error;

use crate::expr_struct::{BinOp, Expr};
use crate::tokentool::{Token, scan_token};

//parse un expréssion :
pub fn parse_expr(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token = Token::Other("");

    let perm = parse_term(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    loop {
        match next_token {
            Token::Number(n) => {
                println!("Erreur de syntaxe le nombre:{n} semble posé problème");
                return Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )));
            }
            Token::Other(str) => {
                if str == "+" || str == "-" {
                    let scaned = parse_term(input)?;
                    input = scaned.0;
                    current_expr =
                        Expr::box_binop_from(current_expr, scaned.1, BinOp::from_str(str));
                }
            }
        }

        if input.is_empty() || input.starts_with(")") {
            return Expr::result_from_current(input, current_expr);
        }

        (input, next_token) = scan_token(input)?;
    }
}

//parse le term suivant :
pub fn parse_term(mut input: &str) -> IResult<&str, Box<Expr>> {
    let perm = parse_factor(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    if input.starts_with(')') {
        return Expr::result_from_current(input, current_expr);
    }
    loop {
        let scaned = scan_token(input)?;
        match scaned.1 {
            Token::Number(a) => {
                println!("Erreur here {a},input::{input}");
            }
            Token::Other(str_token) => {
                if str_token == "+" || str_token == "-" || str_token == ")" {
                    return Expr::result_from_current(input, current_expr);
                } else {
                    (input, _) = scaned;
                    if str_token == "*" || str_token == "/" {
                        let next_factor = parse_factor(input)?;
                        input = next_factor.0;

                        current_expr = Expr::box_binop_from(
                            current_expr,
                            next_factor.1,
                            BinOp::from_str(str_token),
                        );
                    }
                }
            }
        }
        if input.is_empty() || input.starts_with(')') {
            return Expr::result_from_current(input, current_expr);
        }
    }
}

//parse le facteur suivant : {
pub fn parse_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let next_token;
    (input, next_token) = scan_token(input)?;

    match next_token {
        Token::Number(n) => Expr::result_number(input, n),
        Token::Other(str_token) => {
            if str_token == "(" {
                parse_real_factor(input)
            } else if str_token == "-" || str_token == "V" || str_token == "ln" {
                //RECURSIVITÉ :
                let perm = parse_factor(input);
                let (aff_perm, real_perm) = perm?;
                return IResult::Ok((aff_perm, Expr::box_factorop_from(real_perm, str_token)));
            } else {
                return Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )));
            }
        }
    }
}

pub fn parse_real_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let next_token;

    let scaned = parse_expr(input)?;

    input = scaned.0;

    (input, next_token) = scan_token(input)?;

    match next_token {
        Token::Number(_) => Err(nom::Err::Error(Error::new(
            input,
            nom::error::ErrorKind::Digit,
        ))),
        Token::Other(str_token) => {
            if str_token == ")" {
                Expr::result_from_current(input, scaned.1)
            } else {
                Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )))
            }
        }
    }
}
// }
