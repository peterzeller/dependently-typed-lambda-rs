// mod abstractSyntax;
// mod eval;
extern crate pest;
#[macro_use]
extern crate pest_derive;
use crate::eval::eval;
use crate::parser::parse_string;
use std::io::BufRead;
mod abstract_syntax;
mod eval;
mod parser;
#[cfg(test)]
mod test;
use std::io::{self};

fn main() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                let ast = parse_string(&line);
                println!("ast = {:#?}", ast);

                match ast {
                    Err(e) => println!("Could not parse: {}", e),
                    Ok(ast) => {
                        let v = eval(ast);
                        println!("value = {:#?}", v)
                    }
                }
            }
            Err(e) => {
                println!("Could not read from stdin {:?}", e)
            }
        }
    }
}
