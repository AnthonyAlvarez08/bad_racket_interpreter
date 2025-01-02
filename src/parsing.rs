pub mod parsing {
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
        let mut begin_arg : usize = 0;


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

            // make sure it doesn't try to put the function name as an argument
            if begin_arg == 0 {
                begin_arg = cursor;
            }

            // try to push the argument to the string
            if cursor > begin_arg {
                res.push(String::from(  &stg[begin_arg..cursor]  ));
                begin_arg = cursor + 1;
            }
            cursor += 1;
        }

        // if the thing ends like (+ 3 4) it can sometimes miss the 4
        if begin_arg < cursor {
            res.push(String::from(  &stg[begin_arg..stg.len()]  ));
        }
        
        res
    }

}