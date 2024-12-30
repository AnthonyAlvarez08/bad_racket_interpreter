


fn main() {
    println!("Hello, world!");

    let res = parsing::expression_order(String::from("(* (+ (* 4 3) 5) 3)"));

    for i in res {
        println!("parentheses at {} {}", i[0], i[1]);
    }

}


mod parsing {

    // global variables because I don't like having raw literals
    static OPEN_EXPR : char = '(';
    static CLOSE_EXPR : char = ')';

    pub fn expression_order(stg: String) -> Vec<[usize; 2]> {
        

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





}