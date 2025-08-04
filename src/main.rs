use nom::character::complete::space0;
use nom::error::Error;
use nom::{IResult, Parser, branch::alt, bytes::complete::tag, character::complete::digit1};
use std::str::FromStr;
mod mathtools;
use mathtools::*;

//RULES
/*
E : E - T  | E + T
T : F*T    | F/T
F : Number | (E) | -E | lnE | VE

E:Expression
T:Term
F:Factor
*/
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
    //binary operation
    fn box_binop_from(left_box: Box<Expr>, right_box: Box<Expr>, operation: BinOp) -> Box<Expr> {
        Box::new(Expr::BinaryOp {
            left: left_box,
            op: operation,
            right: right_box,
        })
    }

    //factor operation
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
    fn result_number(input: &str, number: f64) -> IResult<&str, Box<Expr>> {
        let result = (input, Box::new(Expr::Number(number)));
        IResult::Ok(result)
    }

    fn result_from_current(input: &str, current_expr: Box<Expr>) -> IResult<&str, Box<Expr>> {
        IResult::Ok((input, current_expr))
    }
}
impl Expr {
    /*Expr to float*/
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

//fonction utilitaire pour scan_token {
pub fn scan_plus(input: &str) -> IResult<&str, &str> {
    tag("+")(input)
}
pub fn scan_moins(input: &str) -> IResult<&str, &str> {
    tag("-")(input)
}
pub fn scan_div(input: &str) -> IResult<&str, &str> {
    tag("/")(input)
}
pub fn scan_fois(input: &str) -> IResult<&str, &str> {
    tag("*")(input)
}
pub fn parens0(input: &str) -> IResult<&str, &str> {
    tag("(")(input)
}
pub fn parens1(input: &str) -> IResult<&str, &str> {
    tag(")")(input)
}
pub fn scan_digit(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

//optionnel !!

pub fn scan_ln(input: &str) -> IResult<&str, &str> {
    tag("ln")(input)
}
pub fn scan_sqrt(input: &str) -> IResult<&str, &str> {
    tag("V")(input)
}

// }

pub fn scantoken(input: &str) -> IResult<&str, &str> {
    alt((
        scan_ln, scan_sqrt, scan_digit, scan_plus, scan_moins, scan_div, scan_fois, parens0,
        parens1, space0,
    ))
    .parse(input.trim())
}

pub fn parse_expr(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token = "";
    let perm = parse_term(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    loop {
        if next_token == "+" || next_token == "-" {
            let scaned = parse_term(input)?;
            input = scaned.0;
            current_expr =
                Expr::box_binop_from(current_expr, scaned.1, BinOp::from_str(next_token));
        }
        if input.is_empty() || input.starts_with(")") {
            //Condition d'arrêt
            return Expr::result_from_current(input, current_expr);
        }
        (input, next_token) = scantoken(input)?;
    }
}

/*----parse le term suivant
    ex:12*4+3 --parse_term--> return 12*4
*/
pub fn parse_term(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token;

    let perm = parse_factor(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    if input.starts_with(')') {
        /*---Retourne car :
            si on a ')',Cela signifie que qu'on déja obtenu le un facteur
                ex : (12) --parsing--> ) => return 12
        */
        return Expr::result_from_current(input, current_expr);
    }
    loop {
        let scaned = scantoken(input)?;
        if scaned.1 == "+" || scaned.1 == "-" {
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
/*----parse le facteur suivant---*/
pub fn parse_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token;

    (input, next_token) = scantoken(input)?;

    if next_token.parse::<f64>().is_ok() {
        let n = f64::from_str(next_token).unwrap();
        Expr::result_number(input, n)
    } else if next_token == "(" {
        let scaned = parse_expr(input)?;
        input = scaned.0;

        (input, next_token) = scantoken(input)?;

        if next_token == ")" {
            return Expr::result_from_current(input, scaned.1);
        } else {
            return Err(nom::Err::Error(Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )));
        }
    } else if next_token == "-" || next_token == "V" || next_token == "ln" {
        //RECURSIVITÉ :
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
    // ENTRÉE / INPUT :
    let a = "(2+3)*2---10";

    // RESULTAT / OUTPUT:
    let v = parse_expr(a);

    match v {
        Ok((rest, expr)) => {
            if rest.is_empty() {
                println!("{:?}", expr);
                let result = expr.eval();
                println!("Result : {:?}", result);
            } else {
                /*
                   ---REMARQUE :
                   Vérifie si l'input est bien vide au final

                   ex:
                       12))) -> invalide (Syntax incorrect on ")))")
                       12    -> valide

                */
                println!("Syntax incorrect on \"{rest}\"");
            }
        }
        Err(_) => {
            println!("Parsing impossible")
        }
    }
}
