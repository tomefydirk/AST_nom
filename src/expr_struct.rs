use nom::IResult;


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
    pub fn from_str(a: &str) -> Self {
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
    pub fn box_binop_from(left_box: Box<Expr>, right_box: Box<Expr>, operation: BinOp) -> Box<Expr> {
        Box::new(Expr::BinaryOp {
            left: left_box,
            op: operation,
            right: right_box,
        })
    }

    //factor operation
    pub fn box_factorop_from(current_expr: Box<Expr>, token: &str) -> Box<Expr> {
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
    pub fn result_number(input: &str, number: f64) -> IResult<&str, Box<Expr>> {
        let result = (input, Box::new(Expr::Number(number)));
        IResult::Ok(result)
    }

    pub fn result_from_current(input: &str, current_expr: Box<Expr>) -> IResult<&str, Box<Expr>> {
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
            Expr::Ln(expr) => expr.eval().ln(),
            Expr::Sqrt(expr) => expr.eval().sqrt(),
        }
    }
}
