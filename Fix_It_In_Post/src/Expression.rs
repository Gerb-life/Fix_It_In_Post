
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
    infix: String,
}

//Implementing the "Objects" methods 
impl Expression {
    
//constructor
    fn new(input: String) -> Self{
        Expression { 
            postfix : input,
            infix: String::new(),
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

    let mut expression_list: Vec<Expression> =  build_expression_list("input.txt").unwrap();//if inside src directory path is input.txt
    
    for i in 0..expression_list.len(){
    solve(&mut expression_list[i]);

    to_string(&expression_list[i]);
    }
    

    

     
    
    

}

/**
 * This function accepts a reference to a string slice representing the input file name
 * and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
 */

 fn build_expression_list(input_file: &str) -> Result<Vec<Expression> , io::Error>{
    let file = File::open(input_file);
    //checking not complete yet , works if file exists
    let check_file = match file{
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut expression_list: Vec<Expression> = Vec:: new();
    let reader = BufReader::new(check_file);
    //initialize expression list with postfix expressions from file

    for lines in reader.lines(){
        let expression = Expression::new(lines.unwrap());

        expression_list.push(expression);
    }
   
    return Ok(expression_list);
}

/**
 * Takes a reference to a vector of Expressions and solves them. No return value.
 */
fn solve_list(expression_list: &mut Vec<Expression>){
    
   
    
}

/**
 * Takes a reference to a vector of Expressions and sorts them based on the value of the
 * expressions solution.
 */
    fn sort_list(expression_list: &mut Vec<Expression>){
        // We should pass the Vector to this method after we've pass it to solve_list()

    }

    /**
    * This takes a reference to a string slice, representing the output file name and a reference
    * to a vector of expressions. This function returns a ’Result’
    */

    fn write_to_file(output: &str , expression_list: &Vec<Expression>){

    }







////ONCE THIS METHOD IS FINISHED, PUT IT BACK IN THE EXPRESSION CLASS IMPLEMENTATION
/// Were are putting it outside so we can test it with main
/// REMEMBER TO GET RID OF THE FUNCTION "postfix"
/// We'll be using the one defined in Expression 

 /**
 * Solves postfix expression and creates infix express
 * returns - none
 */

fn solve(expression: &mut Expression){
    let mut stack:Vec<String> = Vec::new();
    

    for item in expression.postfix.split_whitespace(){
        //if operand push onto stack
        if(!is_operator(item)){
            stack.push(item.to_string());
        }
        //else pop 2 elements, combine and push onto stack
        else{
            let operand1 = stack.pop().unwrap();
            let operand2 = stack.pop().unwrap();
            let combine = format!("({}{}{})" ,operand2 , item , operand1 );
            stack.push( combine ); 

        }
        
        
    }
    //setting expression infix to be equal to clone of first element
    expression.infix = stack[0].clone();
    
    
}



//simple to string method for checking
fn to_string(expression: &Expression){
    println!("Postfix: {}\n Infix: {:?} \n Expr: {:?} \n" , expression.postfix , expression.infix, expression.expr);
}


/**
 * Returns true if the argument is one of the operators we're looking for 
 */
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