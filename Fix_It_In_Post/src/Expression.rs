
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self , BufRead, BufReader, BufWriter, Write};
use std::process::exit;


///The Expression "Object"
/// postfix: A string representing a postfix expression 
/// expr: The expression's numerical solution
/// infix: The infix representation of the expression
pub struct Expression{
    postfix: String,
    expr: Vec<f64>,
    infix: Vec<String>,
}

//Implementing the "Objects" methods 
impl Expression {
    
//constructor
    fn new(input: String) -> Self{
        Expression { 
            postfix : input,
            infix: Vec::new(),
            expr: Vec::new(),
         }
    }

   

}

/**
 * Parses and handles command line argument, and contains the logic and code to run the program.
 * If there is an error writing to the output file, main should print an appropriate error message.
 */

 fn main(){



    let args: Vec<String> = env::args().collect();
    //handles command line arguemnts commented out for testing
   // if args.len() != 3 {
     //   println!("Usage: cargo run --example expr [INPUTFILE] [OUTPUTFILE]");
      //  return;  // acts as exit
    //}

    //let e = Expression::new(args);

    
    
    // setting input file to the first commane line argument 
    //commented out for testing  
    //let input_file: &str = &args[1];

    let postfix:Vec<String> =  build_expression_list("input.txt").unwrap();//if inside src directory path is input.txt
    //let expr: Vec<f64> = Vec::new();
    //let infix: Vec<String> = Vec::new();
    

     
    //printing elements from postfix for testing
    for i in 0..postfix.len(){
        println!("{}",postfix[i])
    }
}

/**
 * This function accepts a reference to a string slice representing the input file name
 * and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
 */

fn build_expression_list(input_file: &str) -> Result<Vec<String> , io::Error>{
    let file = File::open(input_file);
    //checking not complete yet , works if file exists
    let check_file = match file{
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    
    let reader = BufReader::new(check_file);
    let mut expressions: Vec<String> = Vec::new();// each line from the input file is a postfix expression
    //initializing expressions vector
    for lines in reader.lines(){
        expressions.push(lines.unwrap())
    }

    return Ok((expressions));
}

/**
 * Takes a reference to a vector of Expressions and solves them. No return value.
 */
fn solve_list(expr: Vec<f64>){

    for i in 0..expr.len(){
        println!("{}",expr[i]);
    }
}

/**
 * Takes a reference to a vector of Expressions and sorts them based on the value of the
 * expressions solution.
 */
    fn sort_list(expr: Vec<f64>){
        // We should pass the Vector to this method after we've pass it to solve_list()

    }

    /**
    * This takes a reference to a string slice, representing the output file name and a reference
    * to a vector of expressions. This function returns a ’Result’
    */

    fn write_to_file(output: &str , expr: Vec<f64>){

    }







////ONCE THIS METHOD IS FINISHED, PUT IT BACK IN THE EXPRESSION CLASS IMPLEMENTATION
/// Were are putting it outside so we can test it with main
/// REMEMBER TO GET RID OF THE FUNCTION "postfix"
/// We'll be using the one defined in Expression 

 /**
 * Solves postfix expression and creates infix express
 * returns - none
 */

fn solve(postfix: &String){
    
    postfix.trim();//Gets rid of the whitespace in our expression 

    let mut stack: Vec<i16> = Vec::new();

    for character in postfix.chars(){//We'll be iterating over all the characters
        println!("{}", character);

        

    }
}


/**
 * Returns true if the argument is one of the operators we;re looking for 
 */
fn is_operator(){

}