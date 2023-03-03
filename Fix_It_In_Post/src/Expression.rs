use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write, Error};
use std::process::exit;



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
    /// * input - The input 
    fn new(input: String) -> Self{
        Expression { 
            postfix : input,
            infix: String::new(),
            expr: Vec::new(),
         }
    }
 
    ///
    /// Solves postfix expression and creates infix express
    /// 
    fn solve(&mut self){
        let mut stack = Vec::new();
    
        // Splitting over whitespaces gives us all the characters in our expression
        for character in self.postfix.split_whitespace(){
            if is_operator(character){ // If our character is a expression
                    let val2 = stack.pop().unwrap();
                    let val1 = stack.pop().unwrap();
                    let result;

                    match character{
                        "*" => result = val1 * val2,
                        "-" => result = val1 - val2,
                        "+" => result = val1 + val2,
                        "/" => result = val1 / val2,
                        &_ => todo!(), // For some reason, Rust is demanding we put this at the end
                    }
            
                    stack.push(result);

            }else if is_digit(character){// Else our character is a number 
                // We convert the character to f64 and push it on the stack
                let num = character.parse::<f64>().unwrap();
                stack.push(num);
            }
        }
        self.expr = stack;
    }
   
    ///
    ///  Converts 'postfix' to a infix expression.
    /// 
    fn postfix_to_infix(&mut self){
        let mut stack:Vec<String> = Vec::new();
        
        // Gets all of the characters in a expression. 
        for character in self.postfix.split_whitespace(){
            if is_digit(character){ // If operand push onto stack
                stack.push(character.to_string());
                //println!("{:?}", stack);
            }
            // Else pop 2 elements, combine and push onto stack
            else if is_operator(character){
                let operand1 = stack.pop().unwrap();
                let operand2 = stack.pop().unwrap();
                let combine = format!("({} {} {})" ,operand2 , character , operand1 );
                stack.push( combine ); 
            } 
        }
        if stack.len() == 1 { // If we end our loop with 1 value in the stack 
            // Make our expression's 'infix' field our infix expression 
            self.infix = stack[0].clone();
        }
    }
    
    ///
    /// Formats our Epxression, shoing it in infix notation and its value.
    /// 
    fn to_string(& self) -> String{
        let result = format!("{} = {:?}\n" , self.infix , self.expr[0]);
        return result;
    }


    /// We need to RETURN AN ERROR MESSAGE if our expression is invalid 
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
            }else if is_digit(character){
                stack.push(character)
            }
            else{
                valid = false;
            }
        }
        // The stack length should be 1, or this isn't a valid expression
        return valid && stack.len() == 1
    }   
    
}


/// Parses and handles command line argument, and contains the logic and code to run the program.
/// If there is an error writing to the output file, main should print an appropriate error message.
/// 
 fn main(){

    let mut _args: Vec<String> = env::args().collect();
    
    //handles command line arguemnts commented out for testing
    // if args.len() != 3 {
    // println!("Usage: cargo run --example expr [INPUTFILE] [OUTPUTFILE]");
    // return;  // acts as exit
    //}

    //Gets the 2nd index of the argument array (which should be the file we're pulling from)

    
    // setting input file to the first commane line argument 
    //commented out for testing  
    //let input_file: &str = &args[1];
    let input_file = &_args[1];
    //let output_file = &_args[2];
    let mut expression_list: Vec<Expression> =  build_expression_list(input_file).unwrap();//if inside src directory path is input.txt
    
    solve_list(&mut expression_list);
    sort_list(&mut expression_list);
    write_to_file("testOutput.txt", &mut expression_list).expect("Failed to create and write");
    
    for i in 0..expression_list.len(){
        println!("{}" , expression_list[i].to_string());
    }
    

}


/// This function accepts a reference to a string slice representing the input file name
/// and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
/// 
/// * - 'file_name' The name of the file we're pulling expressions from. 
fn build_expression_list(file_name: &String) -> Result<Vec<Expression> , Error>{
    let file = File::open(file_name);
    //checking not complete yet , works if file exists
    let check_file = match file{
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut expression_list: Vec<Expression> = Vec:: new();
    let reader = BufReader::new(check_file);

    //Gets all lines from file
    for lines in reader.lines(){
        let clone = lines.unwrap().clone();
        if clone.len() > 0 {
            let expression = Expression::new(clone);
            if expression.is_valid() == true{
                expression_list.push(expression);//Pushing a line from the file into expression list 
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
        expression_list[i].postfix_to_infix();
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
/// to a vector of expressions. This function returns a ’Result’.
/// 
/// * - 'file_name' The name of the file we're pulling expressions from. 
/// * 'expression_list' - A list of Expression objects.
fn write_to_file(file_name: &str , expression_list: &Vec<Expression>) -> Result<(), Error> {
    let mut created = File::create(file_name)?;

    for expr in expression_list{
        let output = expr.to_string();
        created.write(output.as_bytes())?;
    }
    Ok(())
}


/// Returns true if the argument is one of the operators we're looking for.
/// 
/// * 'character' - The character we're checking. 
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

/// Returns true if the argument is one of the characters we're looking for.
/// 
/// * 'character' - The character we're checking. 
fn is_digit (character: &str) -> bool{
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut result = true;

    for element in character.chars(){
        if !digits.contains(&element){
            result = false;
        }
    }
    return result;
}