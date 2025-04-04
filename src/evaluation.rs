// Last modified by Anthony Alvarez on Feb 13, 2025
/**
 * TODO: COND statement
 * TODO: symbols
*/

pub mod evaluation {
    use std::collections::HashMap;
    use std::num::ParseFloatError;
    use std::str::ParseBoolError;

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
    const WHITESPACE : [&str; 4] = ["\t", "\n", " ", "\r"];
    const BOOLLITERALS : [&str; 4] = ["#t", "#f", "true", "false"];

    /// Basically evaluates a whole program
    /// (currently only supports arithmetic lol)
    pub fn evaluate(prog: &String, var_table: &mut HashMap<String, String>) {
        let end = prog.len();
        let mut cursor : usize = 0;

        // TODO: parsing is kinda jank, pls fix
        while cursor < end {
            // go up to next opening parenthesis
            while prog.chars().nth(cursor).unwrap_or_default() != OPEN_EXPR {
                cursor += 1;

                if cursor >= end {

                    let trimmed = prog.trim();

                    // if there is no parenthesis, check that it exists in the variable table or is empty
                    if var_table.contains_key(trimmed) {
                        match var_table.get(trimmed) {
                            Some(stg) => println!("{}", stg),
                            None => println!("What?")
                        }
                    } 


                    return;
                }
            }
            // find where the current expression ends
            let ender = parsing::find_matching_parenthesis(&prog, cursor);

            // do make sure to include the parenthesis in the expression
            let expression_substr = &prog[cursor..ender+1];


            // print the result
            let mut res = match evaluate_expresion(&String::from(expression_substr), var_table, true) {
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
    fn evaluate_expresion(expr: &String, var_table: &mut HashMap<String, String>, first: bool) -> StringRes {

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

        if parsing::expression_order(&expr).is_err() {
            return Err("parsing error".into());
        }


        // TODO: check if thing exists in variables table
        // TODO: check for symbols
        // for (key, val) in var_table.iter() {
        //     println!("Variable: {key} with value {val}");
        // }
        if var_table.contains_key(expr.trim()) {
            let res = match var_table.get(expr.trim()) {
                Some(stg) => Ok(stg.to_owned()),
                None => Err("How?, I checked that it contains the key".into())
            };

            return res;
        }

        // make sure the expression has at least two letters if not immediately taxable
        // if expr.len() < 3 {
        //     return Err("Expression format is (operand arg arg arg...)".into());
        // }
        


        let mut first_non_space = 1;

        // remove the outside parenthesis from the expression
        // eg: (+ 5 3) goes to + 5 3
        // go up to the next non white space
        while let Some(chr) = expr.chars().nth(first_non_space) {
            if first_non_space >= expr.len() || !WHITESPACE.contains(&String::from(chr).as_str()) {
                break;
            }
            first_non_space += 1;
        }
        
        let expr = String::from(&expr[first_non_space..expr.len() - 1]);

        // save a version with the parenthesis still
        let mut orig = expr.to_owned(); 
        orig.insert(0, OPEN_EXPR);
        orig.insert(orig.len(), CLOSE_EXPR);


        // get the index of the next space
        if let Some(dex) = expr.chars().position(|x| x == ' ') {

            // basically just parse the command and go to the more specific evaluation function
            let command = &expr[..dex];

            if command.eq("define") {

                if !first {
                    return Err("Cannot define within another expression".into());
                }


                let parsed_args = match parsing::parse_args(&orig) {
                    Ok(val) => val,
                    Err(error) => {
                        return Err(error.to_owned());
                    }
                };
                let res = eval_define(&parsed_args, var_table);
                return res;
            }

            // recursively evalute all the arguments inside of it
            // args will not be used after this so the evaluation functions
            // can just take ownership of them
            let mut evalerror: Vec<String> = Vec::new();

            let args : Vec<String> = match parsing::parse_args(&orig) {
                Ok(temp) => {
                    temp
                    .iter()
                    .map(|x| { 
                        match evaluate_expresion(x, var_table, false) {
                            Ok(stg) => { return String::from(stg); },
                            Err(err) => {
                                // just make it not have an argument here
                                evalerror.push(err);
                                return String::from("");
                            }
                        }
                    })
                    .collect()
                }
                Err(error) => {
                    return Err(error);
                }

            };

            if evalerror.len() > 0 {
                let mut stg = String::from("");

                for i in evalerror {
                    stg.push_str(&i);
                    stg.push('\n');
                }

                return Err(stg);
            }


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

        // attempt to parse everything into a float
        let args_res : Vec<Result<f32, ParseFloatError>> = args.iter()
            .map(|x| { 
                x.parse::<f32>()
            })
            .collect();

        // make sure everything parsed into a float correctly
        if !args_res.iter().all(|x| x.is_ok()) {
            return Err("At least one arg is not a number".into());
        }

        // now convert all args to float
        let args : Vec<f32> = args_res.into_iter().map(|x| {
            match x {
                Ok(num) => num,
                Err(err) => 0.0
            }
        }).collect();


        let mut temp_res : f32 = 0.0;


        match operand {
            "+" => { temp_res = args.into_iter().fold(0 as f32, |a, b|  a + b ) }
            "*" => { temp_res = args.into_iter().fold(1 as f32,|a, b|  a * b ) }
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
                if args.len() != 3 {
                    return Err("If statements must be in the format of (if <cond> <then_expr> <else_expr>)".into());
                }
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

        // attempt to parse everything into a float
        let args_res : Vec<Result<f32, ParseFloatError>> = args.iter()
            .map(|x| { 
                x.parse::<f32>()
            })
            .collect();

        // make sure everything parsed into a float correctly
        if !args_res.iter().all(|x| x.is_ok()) {
            return Err("At least one arg is not a number".into());
        }

        // now convert all args to float
        let args : Vec<f32> = args_res.into_iter().map(|x| {
            match x {
                Ok(num) => num,
                Err(err) => 0.0
            }
        }).collect();

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

        // attempt to parse everything into a float
        let args_res : Vec<Result<bool, ParseBoolError>> = args.iter()
            .map(|x| { 
                x.parse::<bool>()
            })
            .collect();

        // make sure everything parsed into a float correctly
        if !args_res.iter().all(|x| x.is_ok()) {
            return Err("At least one arg is not a boolean".into());
        }

        // now convert all args to float
        let args : Vec<bool> = args_res.into_iter().map(|x| {
            match x {
                Ok(num) => num,
                Err(err) => false
            }
        }).collect();

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


    fn eval_define(args: &Vec<String>, var_table: &mut HashMap<String, String>) -> StringRes {
        // TODO: support defining functions

        if args.len() == 2 {
            let res = evaluate_expresion(&args[1].clone(), var_table, false);
            match res {
                Ok(arg) => {
                    let temp = args[0].clone().trim().to_string();
                    let temp2 = &temp.as_str();

                    // filter out invalid variable names
                    if ARITHMETIC.contains(temp2) 
                        || CONDS.contains(temp2) 
                        || WHITESPACE.contains(temp2)
                        || BOOLEAN.contains(temp2)
                        || COMPARISON.contains(temp2)
                        || BOOLLITERALS.contains(temp2)
                        || temp.starts_with(&['0', '9', '8', '7', '6', '5', '4', '3', '2', '1', '[', ']', '{', '}', '#'])
                        || temp.eq("define")
                    {
                        return Err("Invalid variable name".into());
                    }


                    var_table.insert(temp, arg);
                    return Ok("".into());
                }
                Err(err) => {
                    return Err("Something went wrong in evaluating expression".into());
                }
            }
            
        } else {
            Err("Defining of variable should be (define var_name expr)".into())
        }
    }


}