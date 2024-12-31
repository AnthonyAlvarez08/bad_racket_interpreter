
/*
* TODO: Use result types to make error handling
* https://docs.racket-lang.org/htdp-langs/index.html
*/

// temporarily shut up about unused variables
#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};

fn main() {

    // read command line arguments
    // there should only be one, being the path to a racket program
    // if there is none, start a repl
    if let Some(arg1) = env::args().nth(1) {
        println!("Bad Racket Interpreter will execute program {}\n\n", arg1);



        // literally just copied the following from the Rust file IO wiki

        let path = Path::new(&arg1);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {
                if !s.starts_with("#lang racket") {
                    panic!("This is not a racket file");
                }
                // basically just run the whole program
                evaluation::evaluate(&s)
            },
        }
    } else {
        // start a REPL
        println!("Initiating Bad Racket Interpreter REPL, type `quit` or `exit` to quit\n\n");
        loop {
            
            // print thing on the same line
            print!("BRI> ");
            stdout().flush().unwrap();
            let mut buffer = String::new();
            
            if stdin().read_line(&mut buffer).is_ok() {
                let buffer = buffer.trim().to_owned();
                if buffer.eq("exit") ||  buffer.eq("quit") {
                    return;
                }

                evaluation::evaluate(&buffer);
            }

            
        }
        
    }

   



}

mod evaluation {
    use crate::parsing;

    // global variables because I don't like having raw literals
    const OPEN_EXPR : char = '(';
    const CLOSE_EXPR : char = ')';
    const ARITHMETIC : [&str; 4] = ["*", "+", "-", "/"];
    const BOOLEAN : [&str; 10] = ["=", ">", "<", "<=", ">=", "and", "or", "xor", "nand", "nor"];
    const CONDS : [&str; 3] = ["if", "cond", "else"];
    const LITERAL_BOOL : [&str; 2] = ["#t", "#f"];

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

            // do make sure to include the parenthesis in the expression
            let expression_substr = &prog[cursor..ender+1];


            // print the result
            println!("{}", evaluate_expresion(&String::from(expression_substr)));
            
            // go to next expression
            cursor = ender - 1;

        }


    }

    /// calculates the result of an individual expression
    fn evaluate_expresion(expr: &String) -> String {

        // trim any white space in there
        let expr = String::from(expr.trim());

        // if it is just a number or a boolean then just return that
        // serves as base case
        if expr.parse::<f64>().is_ok() || expr.parse::<bool>().is_ok() || LITERAL_BOOL.contains(&expr.as_str())  {
            return expr.to_string();
        }

        // remove the outside parenthesis from the expression
        // eg: (+ 5 3) goes to + 5 3
        let orig = expr.to_owned();
        let expr = String::from(&expr[1..expr.len() - 1]);

        

        // get the index of the next space
        if let Some(dex) = expr.chars().position(|x| x == ' ') {

            // basically just parse the command and go to the more specific evaluation function
            let command = &expr[..dex];

            // recursively evalute all the arguments inside of it
            // args will not be used after this so the evaluation functions
            // can just take ownership of them
            let args : Vec<String> = parsing::parse_args(&orig).iter().map(|x| { evaluate_expresion(x) }).collect();

            // return String::from(command);

            if ARITHMETIC.contains(&command) {
                return eval_arithmetic(command, args);
            }
            
            if BOOLEAN.contains(&command) {
                return eval_boolean(command, args);
            }



            // otherwise just return this command I gues
            panic!("Invalid symbol `{}`", command);
        } else {
            panic!("Invalid syntax");
        }
        
        
    }


    fn eval_arithmetic(operand: &str, args: Vec<String>) -> String {
        let args :Vec<f32> = args.iter()
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

    fn eval_boolean(operand: &str, args: Vec<String>) -> String {
        


        String::from("")
    }

}

mod parsing {
    use core::panic;


    // global variables because I don't like having raw literals
    const OPEN_EXPR : char = '(';
    const CLOSE_EXPR : char = ')';
    const WHITESPACE : [&str; 4] = ["\t", "\n", " ", "\r"];

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
            // if only one open expression and we found a closer, than we're done
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
    /// EX: (+ 2 (+ 1 1) (* 4 5) (/ 15 (+ 2 1)))
    /// would return {2, (+ 1 1), (* 4 5), (/ 15 (+ 2 1))}
    pub fn parse_args(stg: &String) -> Vec<String> {
        // so problem is to find each white space not inside a parenthesis

        // remove the outside parenthesis from the expression
        // eg: (+ 5 3) goes to + 5 3
        let stg = String::from(&stg[1..stg.len() - 1]);
        let mut res : Vec<String> = Vec::new();
        
        // lets me know where parentheses start and end
        let parentheses = expression_order(&stg);


        let mut cursor : usize = 1;
        let mut begin_arg : usize = 2;


        while cursor < stg.len() {
            // scroll up to the next white space
            while let Some(chr) = stg.chars().nth(cursor) {
                if WHITESPACE.contains(&String::from(chr).as_str()) {
                    break;
                }
                cursor += 1;
            }


            if cursor >= stg.len() {
                break;
            }

            // if it is inside a pair of parenthesis, then skip
            let mut skip = false;
            for i in parentheses.iter() {
                if cursor >= i[0] && cursor <= i[1] {
                    skip = true;
                    break;
                }
            }
            if skip {
                cursor += 1;
                continue;
            }

            // try to push the argument to the string
            if cursor > begin_arg {
                res.push(String::from(  &stg[begin_arg..cursor]  ));
                begin_arg = cursor + 1;
            }
            cursor += 1;
        }

        if begin_arg < cursor {
            res.push(String::from(  &stg[begin_arg..stg.len()]  ));
        }
        
        res
    }

}