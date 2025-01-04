// Last modified by Anthony Alvarez on Jan 2, 2025
/**
 * TODO: make sure it doesn't crash if you do something stupid like (+ 1 2 3 "banana")
 * TODO: use result types so that the interpreter doesn't crap itself if it encounters a syntax error
*/

pub mod evaluation {
    use crate::parsing::parsing;

    // I dont want to type it out every time
    type StringRes = Result<String, String>;


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
            let mut res = match evaluate_expresion(&String::from(expression_substr)) {
                Ok(stg) => stg,
                Err(err) => { 
                    println!("Error: {}", err);
                    return; 
                }
            };

            if res.trim().eq("true") {
                res = "#t".to_owned();
            }
            if res.trim().eq("false") {
                res = "#f".to_owned();
            }
            println!("{}", res);
            
            // go to next expression
            cursor = ender - 1;

        }


    }

    /// calculates the result of an individual expression
    fn evaluate_expresion(expr: &String) -> StringRes {

        // trim any white space in there
        let expr = String::from(expr.trim());

        // if it is just a number or a boolean then just return that
        // serves as base case
        if expr.parse::<f64>().is_ok() || expr.parse::<bool>().is_ok()  {
            return Ok(expr.to_string());
        } else if expr.trim().eq("#t") {
            return Ok(String::from("true"));
        } else if expr.trim().eq("#f") {
            return Ok(String::from("false"));
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
            let args : Vec<String> = parsing::parse_args(&orig)
                .iter()
                .map(|x| { 
                    match evaluate_expresion(x) {
                        Ok(stg) => { return String::from(stg); },
                        Err(err) => {
                            // just make it not have an argument here
                            return String::from("");
                        }
                    }
                })
                .collect();

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
            return Err(format!("Invalid symbol `{command}`").into()); // string interpolation?
        } else {
            return Err("Invalid syntax".into());
        }
        
        
    }


    fn eval_arithmetic(operand: &str, args: Vec<String>) -> StringRes {
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
                    return Err("Modulo only takes two arguments".into());
                }
                temp_res = args[0] % args[1];
            }
            "sqrt" => {
                if args.len() != 1 {
                    return Err("Sqrt only takes one argument".into());
                }
                temp_res = f32::sqrt(args[0]);
            }
            _ => {}
        }


        Ok(temp_res.to_string())
    }


    /// DONT USE, DEPENDS ON EVAL BOOLEAN
    fn eval_cond(operand: &str, args: Vec<String>) -> StringRes {
        match operand {
            "if" => {
                match args[0].parse::<bool>() {
                    Ok(res) => {
                        if res {
                            return Ok(args[1].to_owned());
                        } else {
                            return Ok(args[2].to_owned());
                        }
                    }
                    Err(msg) => {
                        return Err("Conditional condition doesn't evaluate to a boolean".into());
                    }
                }
            }
            "cond" => {
                return Err("Not implemented".into());
            }
            _ => { return Err("What? How did you manage to get to evaluate condition without a condition clause".into()) }
        }
    }

    fn eval_comparison(operand: &str, args: Vec<String>) -> StringRes {

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

        Ok(temp_res.to_string())

    }

    fn eval_boolean(operand: &str, args: Vec<String>) -> StringRes {

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
                    return Err("XOR only takes two arguments".into());
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
                    return Err("NOT only takes one argument".into());
                }

                temp_res = !args[0];
            }
            _ => {}

        }
        Ok(temp_res.to_string())
    }

}