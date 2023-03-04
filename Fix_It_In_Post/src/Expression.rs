/// Gabriel Rodriguez and Josiah Cherbony 
/// Version: Spring 2023
/// 
/// Description: A program that takes a file of postfix expressions as input
/// and returns a sorted list of infix expressions in a seperate file. 

use std::env;
use std::fs::{File};
use std::io::{BufRead, BufReader, Write, Error as Err};

/// The Expression "Object"
/// 
/// * 'postfix' - A string representing a postfix expression 
/// * 'expr' - The expression's numerical solution
/// * 'infix' - The infix representation of the expression
pub struct Expression{
    postfix: String,
    expr: Vec<f64>,
    infix: String,
}

//Implementing the "Objects" methods 
impl Expression {
    
    /// Constructs a new 'Expression' object 
    /// 
    /// * 'input' - The postfix expression 
    /// 
    /// Return:
    /// 
    /// A new Expression object.
    fn new(input: String) -> Self{
        Expression { 
            postfix : input,
            infix: String::new(),
            expr: Vec::new(),
         }
    }
 
    ///
    /// Solves our postfix expression and creates infix expression.
    /// Both are asssigned to our objects' fields.
    /// 
    fn solve(&mut self){
        // We have 2 stacks, the 'expr' wich will hold our expression's value
        // and 'stack' which will the infix notation. 
        let mut stack:Vec<String> = Vec::new();
        

        // Splitting over whitespaces gives us all the characters in our expression
        for character in self.postfix.split_whitespace(){

            if is_operator(character){ // If our character is an operator 
                // Getting the current 2 values in our stack 
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                let val2 = self.expr.pop().unwrap();
                let val1 = self.expr.pop().unwrap();
                let result;
                
                match character{
                    "*" => result = val1 * val2,
                    "-" => result = val1 - val2,
                    "+" => result = val1 + val2,
                    "/" => result = val1 / val2,
                    &_ => todo!(), 
                }
              
                let combine = format!("({} {} {})" , operand1, character, operand2); 
                
                stack.push(combine); 
                self.expr.push(result);

            }else if is_num(character){// Else our character is a number 
                // We convert the character to f64 and push it on the stack
                let num = character.parse::<f64>().unwrap();
                self.expr.push(num);
                stack.push(character.to_string());
            }
        }
        if stack.len() == 1 { // If we end our loop with 1 value in the stack 
            let mut string1 = stack[0].to_string();
            // This removes the outter paranthesis from our expression
            if string1.contains("("){
                string1 = string1[1..string1.len() - 1].to_string();
            }
            self.infix = string1
            // Make our expression's 'infix' field our infix expression 
        }
    }
   
    /// A method that returns true if a postfix expression is valid, and false if it's  not. 
    /// 
    /// Return:
    /// 
    /// True if the expression is a valid postfix expression. Otherwise, false. 
    fn is_valid(&self) -> bool{
        let mut valid = true;
        let mut stack:Vec<&str> = Vec::new();

        for character in self.postfix.split_whitespace(){
            if is_operator(character){
                // If we encournter an operator, we should have at least 2 values in out stack
                if stack.len() >= 2 { 
                    stack.pop();
                    stack.pop();
                    stack.push("0");//We can just push some random value
                }
                else { // Else this is an invalid expression
                    valid = false;
                }
            }else if is_num(character){
                stack.push(character)
            }
            else{ // Else if we come across an invalid character
                valid = false;
            }
        }
        // The stack length should be 1, or this isn't a valid expression
        return valid && stack.len() == 1
    }   

    /// Formats our Epxression, showing it in infix notation and its value.
    /// 
    /// Return:
    /// 
    /// A string of the expression in infix notation and its value. 
    fn to_string(& self) -> String{
        let mssg;

        // Removes trailing 0s from 'expr' that are integers 
        if self.expr[0].floor() == self.expr[0]{
            mssg = format!("{} = {:.0}\n" , self.infix, self.expr[0]);
        }else{
            mssg = format!("{} = {:?}\n" , self.infix, self.expr[0]);
        }

        return mssg;
    }
    
}

///
/// Parses and handles command line argument, and contains the logic and code to run the program.
/// If there is an error writing to the output file, main should print an appropriate error message.
/// 
 fn main(){
    // Getting our command line arguments 
    let args: Vec<String> = env::args().collect();
    
    // We must have EXACTLY 3 arguments 
    if args.len() != 3 {
        println!("Usage: cargo run --example expr [INPUTFILE] [OUTPUTFILE]");
        return;  // acts as exit
    }

    // Setting file names to command line arguments 
    let in_file = &args[1];
    let out_file = &args[2];
    // Create a list of expressions 
    let mut expression_list: Vec<Expression> =  build_expression_list(in_file)
        .expect("Failed to build expression list!");
    
    solve_list(&mut expression_list); // Solves our list of expressions 
    sort_list(&mut expression_list); // Sorts our expressions by output
    write_to_file(out_file, &mut expression_list).expect("Failed to create file!");
}


/// This function accepts a reference to a string slice representing the input file name
/// and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
/// 
/// * - 'file_name' The name of the file we're pulling expressions from. 
/// 
/// Return:
/// 
/// A list of valid postfix expression, or error wrapped in a Result.
fn build_expression_list(file_name: &String) -> Result<Vec<Expression> , Err>{
    let file = File::open(file_name);
    // Checking not complete yet, works if file exists
    let check_file = match file{
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut expression_list: Vec<Expression> = Vec:: new();
    let reader = BufReader::new(check_file);

    // Gets all lines from file
    for lines in reader.lines(){
        let clone = lines.unwrap().clone();
        if clone.len() > 0 { // The string must have more than 0 characters 
            let expression = Expression::new(clone);
            
            if expression.is_valid(){ // If the expression is valid 
                expression_list.push(expression);//Pushing a line from the file into expression list 
            }
            else{// Else the expression is invalid
                println!("Error! '{:}' is not a valid expression!", expression.postfix);
            }
        }
    }
    return Ok(expression_list);
}

/// Takes a reference to a vector of Expressions and solves them. 
/// 
/// * 'expression_list' - A list of Expression objects
fn solve_list(expression_list: &mut Vec<Expression>){
    for i in 0..expression_list.len(){
        expression_list[i].solve();
    } 
}

/// Takes a reference to a vector of Expressions and sorts them based on the value of the
/// expressions solution.
/// 
/// * 'expression_list' - A list of Expression objects
fn sort_list(expression_list: &mut Vec<Expression>){
    // We should pass the Vector to this method after we've pass it to solve_list()
    let length = expression_list.len();

    for i in 0..length{
        // Finds the smallest expression value in our current length ('i')
        let smallest = find_smallest(expression_list, i); 
        expression_list.swap(smallest , i);
    }
}

/// A helper method for finding the smallest return value in a expression list
/// within a specified index. 
/// 
/// * 'expression_list' - A list of Expression objects.
/// * 'current' - A number representing the [0..i] list we can traverse.
/// 
/// Return:
/// 
/// The index of the expression with the smallest value.
fn find_smallest(expression_list: &mut Vec<Expression>, current: usize)-> usize{
    let length = expression_list.len(); 
    let mut smallest = current;
        
    //If vector at 'i' is less than smallest then change smallest
    for i in (current +1)..length{
        if expression_list[i].expr[0] < expression_list[smallest].expr[0]{
            smallest = i;
        }
    }  
    smallest
}


/// This takes a reference to a string slice, representing the output file name and a reference
/// to a vector of expressions. 
/// 
/// * 'file_name' - The name of the file we're pulling expressions from. 
/// * 'expression_list' - A list of Expression objects.
/// 
/// Return:
/// 
/// A result that represents a ok or failure.
fn write_to_file(file_name: &str , expression_list: &Vec<Expression>) -> Result<(), Err> {
    let mut file = File::create(file_name)?;

    for expr in expression_list{
        let output = expr.to_string();
        file.write(output.as_bytes())?;
    }
    Ok(())
}

/// Returns true if the argument is one of the operators we're looking for.
/// 
/// * 'character' - The character we're checking. 
/// 
/// Return:
/// 
/// True if the string slice is a defined operator. Otherwise, false.
fn is_operator(character: &str) -> bool{
    let operators = ["/" , "+" , "-" , "*"];
    let mut result = false;

    for i in 0..operators.len(){
        if character == operators[i]{
            result = true;
        }
    }
    return result;
}

/// Returns true if the argument is a number
/// 
/// * 'character' - The character we're checking. 
/// 
/// Return:
/// 
/// True if the string slice is a defined number. Otherwise, false.
fn is_num (character: &str) -> bool{
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let mut result = true;

    for element in character.chars(){
        if !digits.contains(&element){
            result = false;
        }
    }
    return result;
}