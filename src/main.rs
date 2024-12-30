use std::collections::HashMap;


// static mut FunctionTable : HashMap<String, >

fn main() {
    println!("Hello, world!");

    let prog = String::from("(* (+ (* 4 3) 5) 3)");
    let res = parsing::expression_order(&prog);

    for i in res {
        println!("parentheses at {} {}", i[0], i[1]);

        println!("Matching parentheses for {} is at {}", i[0], parsing::find_matching_parenthesis(&prog, i[0]))
    }

}

mod evaluation {
    const ARITHMETIC : [&str; 4] = ["*", "+", "-", "+"];
    const BOOL : [&str; 10] = ["=", ">", "<", "<=", ">=", "and", "or", "xor", "nand", "nor"];
    const CONTROL_FLOW : [&str; 3] = ["if", "cond", "else"];


    pub fn evaluate(prog: String) {
        
    }

    fn evaluate_expresion(expr: String) {

    }
}

mod parsing {
    use core::panic;


    // global variables because I don't like having raw literals
    static OPEN_EXPR : char = '(';
    static CLOSE_EXPR : char = ')';

    pub fn expression_order(stg: &String) -> Vec<[usize; 2]> {
        

        let mut stack : Vec<usize> = Vec::new();
        let mut parentheses_eval_order : Vec<[usize; 2]> = Vec::new();

        for i in 0..stg.len() {

            // println!("Current stack: {}", stack);
            // println!("Current order: {}", parentheses_eval_order);

            if let Some(current) = stg.chars().nth(i)
            {
                if current == OPEN_EXPR {
                    stack.push(i);
                }
                else if current == CLOSE_EXPR {
                    let opener = stack.pop().expect("Unmatched parenthesis");
                    parentheses_eval_order.push( [opener, i]);
                    
                }
            }
           
        }

        if stack.len() != 0 {
            panic!("Mismatched parentheses");
        }

        parentheses_eval_order


    }

    pub fn find_matching_parenthesis(stg: &String, begin: usize) -> usize {
        let mut num_open = 0;

        for i in begin..stg.len() {
            let at = stg.chars().nth(i).unwrap_or_default();
            if at == OPEN_EXPR {
                num_open += 1;
            }
            else if at == CLOSE_EXPR && num_open == 1 {
                return i;
            }
            else if at == CLOSE_EXPR {
                num_open -= 1;
            }
        }
        begin
    }



}