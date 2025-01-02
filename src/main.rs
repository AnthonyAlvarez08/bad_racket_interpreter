
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
mod evaluation;
mod parsing; // for some reason I need to include this here for evaluation.rs to be included properly

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
                evaluation::evaluation::evaluate(&s)
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

                evaluation::evaluation::evaluate(&buffer);
            }

            
        }
        
    }


}

