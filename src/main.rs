mod expr_function;
mod expr_struct;
mod tokentool;

use crate::expr_function::{parse_expr};
//RULES
/*
E : T - T  | T + T | T
T : P*P    | P/P | P
P : F^F    | F
F : Number | (E) | -E | functionE

E:Expression
T:Term
P:Power
F:Factor

function : ln | sqrt | cos | sin | abs
*/

fn main() {
    // ENTRÉE / INPUT :
    let a = "(2)(3)";

    // RESULTAT / OUTPUT:
    let v = parse_expr(a);
    /*
       vous pouver aussi tester:


            let v = parse_term(a);
                   ou
            let v = parse_factor(a);
                   ou 
            let v = parse_power(a);

       quels est la différence d'après vous ?
    */

    match v {
        Ok((rest, expr)) => {
            println!("Expr : {:?}", expr);
            let result = expr.eval();
            println!("Result : {}", result);
            if !rest.is_empty() {
                println!("input_reste : \"{rest}\"");
            }
        }
        Err(_) => {
            println!("Parsing impossible")
        }
    }
}
