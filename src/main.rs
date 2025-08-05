use nom::error::Error;
use nom::{IResult};
mod stringtool;
use stringtool::{Token, scan_token};

//RULES
/*
E : E - T  | E + T | T
T : F*T    | F/T | F
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
            Expr::Ln(expr) =>expr.eval().ln(),
            Expr::Sqrt(expr) => expr.eval().sqrt(),
        }
    }
}

pub fn parse_expr(mut input: &str) -> IResult<&str, Box<Expr>> {
    let mut next_token = Token::Other("");
    let perm = parse_term(input);
    let (aff_perm, real_perm) = perm?;

    let mut current_expr: Box<Expr> = real_perm;

    input = aff_perm;

    loop {
        match next_token {
            Token::Number(_) => {
                println!("Erreur de syntaxe");
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
            //Condition d'arrêt
            return Expr::result_from_current(input, current_expr);
        }
        (input, next_token) = stringtool::scan_token(input)?;
    }
}

/*----parse le term suivant
    ex:12*4+3 --parse_term--> return 12*4
*/
pub fn parse_term(mut input: &str) -> IResult<&str, Box<Expr>> {
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
        let scaned = stringtool::scan_token(input)?;
        match scaned.1 {
            Token::Number(a) => {
                println!("Erreur here {a},input::{input}");
            },
            Token::Other(str_token) => {
                if str_token == "+" || str_token == "-" {
                    return Expr::result_from_current(input, current_expr);
                } else {
                    (input, _) = scaned;

                    if str_token == "*" || str_token == "/" {
                        let next_factor=parse_factor(input)?;
                        input=next_factor.0;
                        current_expr = Expr::box_binop_from(
                            current_expr,
                            next_factor.1,
                            BinOp::from_str(str_token),
                        );
                    }
                }
            }
        }

        if input.is_empty() {
            return Expr::result_from_current(input, current_expr);
        }
    }
}
pub fn parse_real_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let next_token;
    let scaned = parse_expr(input)?;
    input = scaned.0;

    (input, next_token) = scan_token(input)?;

    match next_token {
        Token::Number(_) => {
           Err(nom::Err::Error(Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )))
        }
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
/*----parse le facteur suivant---*/
pub fn parse_factor(mut input: &str) -> IResult<&str, Box<Expr>> {
    let  next_token;

    (input, next_token) = scan_token(input)?;

    match next_token {
        Token::Number(n) => {
           Expr::result_number(input, n)
        }
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
fn main() {
    // ENTRÉE / INPUT :
    let a = "(2+3)*2---10.01";

    // RESULTAT / OUTPUT:
    let v = parse_expr(a);
    /*
       vous pouver aussi tester:


           let v = parse_term(a);
                   ou
           let v = parse_factor(a);

       quels est la différence d'après vous ?
    */

    match v {
        Ok((rest, expr)) => {
            println!("{:?}", expr);
            let result = expr.eval();
            println!("Result : {:?}", result);
            if !rest.is_empty() {
                println!("input_reste : \"{rest}\"");
            }
        }
        Err(_) => {
            println!("Parsing impossible")
        }
    }
}
