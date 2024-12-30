
/*
* TODO: finish parse args function
*/

fn main() {
    println!("Hello, world!");

    // let prog = String::from("(* (+ (* 4 3) 5) 3)");
    // let res = parsing::expression_order(&prog);

    // for i in res {
    //     println!("parentheses at {} {}", i[0], i[1]);

    //     println!("Matching parentheses for {} is at {}", i[0], parsing::find_matching_parenthesis(&prog, i[0]))
    // }

    let prog2 = "#lang racket

(+ 5 4)
(* 7 3)
(/ (* 3 5) 5)
(+ (* 4 3) 5)
(* (+ (* 4 3) 5) 3)";

    evaluation::evaluate(&String::from(prog2));

    

}

mod evaluation {
    use crate::parsing;

    // global variables because I don't like having raw literals
    const OPEN_EXPR : char = '(';
    const CLOSE_EXPR : char = ')';
    const ARITHMETIC : [&str; 4] = ["*", "+", "-", "+"];
    const BOOLEAN : [&str; 10] = ["=", ">", "<", "<=", ">=", "and", "or", "xor", "nand", "nor"];
    const CONDS : [&str; 3] = ["if", "cond", "else"];

    /// Basically evaluates a whole program
    /// (currently only supports arithmetic lol)
    pub fn evaluate(prog: &String) {
        let end = prog.len();
        let mut cursor : usize = 0;

        while cursor < end {
            // go up to next opening parenthesis
            while prog.chars().nth(cursor).unwrap_or_default() != OPEN_EXPR {
                cursor += 1;

                if cursor >= end {
                    return;
                }
            }
            // find where the current expression ends
            let ender = parsing::find_matching_parenthesis(&prog, cursor);

            // do not include the parenthesis inside the expression lol
            let expression_substr = &prog[cursor+1..ender];


            // print the result
            println!("{}", evaluate_expresion(&String::from(expression_substr)));
            
            // go to next expression
            cursor = ender - 1;

        }


    }

    /// calculates the result of an individual expression
    fn evaluate_expresion(expr: &String) -> String {
        // get the index of the next space
        if let Some(dex) = expr.chars().position(|x| x == ' ') {

            // basically just parse the command and go to the more specific evaluation function

            let command = &expr[..dex];

            if ARITHMETIC.contains(&command) {
                return eval_arithmetic(command, &expr);
            }
            
            if BOOLEAN.contains(&command) {
                return eval_boolean(command, &expr);
            }

            if CONDS.contains(&command) {
                return eval_conditionals(&expr);
            }

            // otherwise just return this command I gues
            panic!("Invalid symbol `{}`", command);
        } else {
            panic!("Invalid syntax");
        }
        
        
    }

    fn eval_arithmetic(operand: &str, expr: &String) -> String {
        let args :Vec<f32> = parsing::parse_args(expr)
            .iter()
            .map(|x| { x.parse::<f32>().unwrap() })
            .collect();

        let mut temp_res : f32 = 0.0;

        match operand {
            "+" => { temp_res = args.into_iter().reduce(|a, b|  a + b ).unwrap() }
            "*" => { temp_res = args.into_iter().reduce(|a, b|  a * b ).unwrap() }
            "/" => { temp_res = args.into_iter().reduce(|a, b|  a / b ).unwrap() }
            "-" => { temp_res = args.into_iter().reduce(|a, b|  a - b ).unwrap() }
            _ => {}
        }


        temp_res.to_string()
    }

    fn eval_boolean(operand: &str, expr: &String) -> String {
        let args = parsing::parse_args(expr);


        String::from("")
    }

    fn eval_conditionals(expr: &String) -> String {


        String::from("")
    }
}

mod parsing {
    use core::panic;


    // global variables because I don't like having raw literals
    static OPEN_EXPR : char = '(';
    static CLOSE_EXPR : char = ')';

    /// heuristic to see in which order things are going to be evaluated in
    /// basically just the matching parenthesis algorithm
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


    /// given a string and the location of an opening parenthesis
    /// it will find the location in the string of the matching closing parenthesis
    /// returns 0 if it can find it
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
        0
    }

    /// NOT YET IMPLEMENTED
    /// given a string in the form of 
    /// operand arg arg arg...
    /// it will give the operands in a vector form
    /// note that operands can be other expressions
    /// EX: + (+ 1 1) (* 4 5) (/ 15 (+ 2 1))
    /// would return {(+ 1 1), (* 4 5), (/ 15 (+ 2 1))}
    pub fn parse_args(stg: &String) -> Vec<String> {
        Vec::new()
    }

}