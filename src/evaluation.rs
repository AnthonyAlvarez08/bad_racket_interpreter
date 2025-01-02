// Last modified by Anthony Alvarez on Jan 2, 2025


pub mod evaluation {
    use crate::parsing::parsing;


    // global variables because I don't like having raw literals
    const OPEN_EXPR : char = '(';
    const CLOSE_EXPR : char = ')';
    const ARITHMETIC : [&str; 6] = ["*", "+", "-", "/", "modulo", "sqrt"];
    const BOOLEAN : [&str; 6] = ["and", "or", "xor", "nand", "nor", "not"];
    const COMPARISON : [&str; 5] = ["=", ">", "<", "<=", ">="];
    const CONDS : [&str; 2] = ["if", "cond"];

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
        if expr.parse::<f64>().is_ok() || expr.parse::<bool>().is_ok()  {
            return expr.to_string();
        } else if expr.trim().eq("#t") {
            return String::from("true");
        } else if expr.trim().eq("#f") {
            return String::from("false");
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

            if COMPARISON.contains(&command) {
                return eval_comparison(command, args);
            }

            if CONDS.contains(&command) {
                return eval_cond(command, args);
            }



            // otherwise just return this command I gues
            panic!("Invalid symbol `{}`", command);
        } else {
            panic!("Invalid syntax");
        }
        
        
    }


    fn eval_arithmetic(operand: &str, args: Vec<String>) -> String {
        let args : Vec<f32> = args.iter()
            .map(|x| { x.parse::<f32>().unwrap() })
            .collect();

        let mut temp_res : f32 = 0.0;

        match operand {
            "+" => { temp_res = args.into_iter().reduce(|a, b|  a + b ).unwrap() }
            "*" => { temp_res = args.into_iter().reduce(|a, b|  a * b ).unwrap() }
            "/" => { temp_res = args.into_iter().reduce(|a, b|  a / b ).unwrap() }
            "-" => { 
                if args.len() == 1 {
                    temp_res = -1.0 * args[0];
                } else {
                    temp_res = args.into_iter().reduce(|a, b|  a - b ).unwrap();
                }
                
            }
            "modulo" => {
                if args.len() != 2 {
                    panic!("Modulo only takes two arguments");
                }
                temp_res = args[0] % args[1];
            }
            "sqrt" => {
                if args.len() != 1 {
                    panic!("Sqrt only takes one argument");
                }
                temp_res = f32::sqrt(args[0]);
            }
            _ => {}
        }


        temp_res.to_string()
    }


    /// DONT USE, DEPENDS ON EVAL BOOLEAN
    fn eval_cond(operand: &str, args: Vec<String>) -> String {
        match operand {
            "if" => {
                match args[0].parse::<bool>() {
                    Ok(res) => {
                        if res {
                            return args[1].to_owned();
                        } else {
                            return args[2].to_owned();
                        }
                    }
                    Err(msg) => {
                        panic!("Conditional condition doesn't evaluate to a boolean");
                    }
                }
            }
            "cond" => {
                panic!("Not implemented");
            }
            _ => { panic!("What? How did you manage to get to evaluate condition without a condition clause") }
        }
    }

    fn eval_comparison(operand: &str, args: Vec<String>) -> String {

        let args : Vec<f32> = args.iter()
            .map(|x| { x.parse::<f32>().unwrap() })
            .collect();

        let mut temp_res : bool = false;


        // ["=", ">", "<", "<=", ">="]
        // all of these have arbitrary arguments
        // all operate on numbers only
        match operand {
            "=" => {
                // check all are equal
                temp_res = true;
                for i in 1..args.len() {
                    temp_res = temp_res & (args[i - 1] == args[i]); 
                }
            }

            ">" => { 
                temp_res = true;
                for i in 1..args.len() {
                    temp_res = temp_res & (args[i - 1] > args[i]); 
                }
            }

            "<" => { 
                temp_res = true;
                for i in 1..args.len() {
                    temp_res = temp_res & (args[i - 1] < args[i]); 
                }
            }

            "<=" => { 
                temp_res = true;
                for i in 1..args.len() {
                    temp_res = temp_res & (args[i - 1] <= args[i]); 
                }
                
            }
            ">=" => {
                temp_res = true;
                for i in 1..args.len() {
                    temp_res = temp_res & (args[i - 1] >= args[i]); 
                }
            }
            _ => {}
        }

        temp_res.to_string()

    }

    fn eval_boolean(operand: &str, args: Vec<String>) -> String {

        let args : Vec<bool> = args.iter()
            .map(|x| { x.parse::<bool>().unwrap() })
            .collect();

        let mut temp_res : bool = false;

        match operand {
            "and" => {
                // takes arbitrary number of args
                temp_res = args.into_iter().reduce(|a, b|  a & b ).unwrap()
            }
            "or" => {
                // takes arbitrary number of args
                temp_res = args.into_iter().reduce(|a, b|  a | b ).unwrap()
            }
            "xor" => {
                // only two args
                if args.len() != 2 {
                    panic!("XOR only takes two arguments");
                }

                temp_res = args[0] ^ args[1];
            }
            "nand" => {
                // takes arbitrary number of args
                temp_res = !args.into_iter().reduce(|a, b|  a & b ).unwrap()
            }
            "nor" => {
                // takes arbitrary number of args
                temp_res = !args.into_iter().reduce(|a, b|  a & b ).unwrap()
            }
            "not" => {
                // only one arg
                if args.len() != 1 {
                    panic!("NOT only takes one argument");
                }

                temp_res = !args[0];
            }
            _ => {}

        }
        temp_res.to_string()
    }

}