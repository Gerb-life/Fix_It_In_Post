
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write, Error};
use std::process::exit;



/**
 * Parses and handles command line argument, and contains the logic and code to run the program.
 * If there is an error writing to the output file, main should print an appropriate error message.
 */

fn main(){
    let postfix = String::new();
    let expr: Vec<f64> = Vec::new();
    let infix: Vec<String> = Vec::new();
}


//constructor
fn new(input: String){

}

/**
 * Solves postfix expression and creates infix express
 * returns - none
 */

fn solve(postfix: &String){

}


/**
 * This function accepts a reference to a string slice representing the input file name
 * and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
 */

fn build_expression_list(input_file: &str){
    
}

/**
 * Takes a reference to a vector of Expressions and solves them. No return value.
 */

fn solve_list(expr: Vec<f64>){

}
/**
 * Takes a reference to a vector of Expressions and sorts them based on the value of the
 *   expressions solution.
 */
fn sort_list(expr: Vec<f64>){

}

/**
 * This takes a reference to a string slice, representing the output file name and a reference
 * to a vector of expressions. This function returns a ’Result’
 */

fn write_to_file(output: &str , expr: Vec<f64>){

}

