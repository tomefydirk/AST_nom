
mod tokentool;
mod expr_function;
mod expr_struct;
use expr_function::{parse_expr};
//RULES
/*
E : E - T  | E + T | T
T : F*T    | F/T | F
F : Number | (E) | -E | lnE | VE

E:Expression
T:Term
F:Factor
*/

fn main() {
    // ENTRÉE / INPUT :
    let a = "  (ln(2) -   ln(2) ) --1";

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
            println!("Expr : {:?}", expr);
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
