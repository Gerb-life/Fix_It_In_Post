
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self , BufRead, BufReader, BufWriter, Write};
use std::process::exit;
use evalexpr::*; //Ask Kreahling if we can use this?


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

/**
 * Solves postfix expression and creates infix express
 * returns - none
 */

fn solve(&mut self){
    let mut stack = Vec::new();
    let expressions = "*+/-";
    
    // Splitting over whitespaces gives us all the characters in our expression
    for character in self.postfix.split_whitespace(){
        if expressions.contains(character){
            let val2 = stack.pop().unwrap();
            let val1 = stack.pop().unwrap();
            let mut result = 0.0;

            match character{
                "*" => result = val1 * val2,
                "-" => result = val1 - val2,
                "+" => result = val1 + val2,
                "/" => result = val1 / val2,
                &_ => todo!(), //For some reason< Rust is demanding we put this at the end
            }
            
            stack.push(result);

        }else{//Else our character is a number 
            //We convert the character to f64 and push it on the stack
            let num = character.parse::<f64>().unwrap();
            stack.push(num);
        }

    }
    self.expr = stack;
    //print!("{:?}", stack);


}
   

fn postfix_to_infix(&mut self){
    let mut stack:Vec<String> = Vec::new();
    

    for item in self.postfix.split_whitespace(){
        //if operand push onto stack
        if(!is_operator(item)){
            stack.push(item.to_string());
        }
        //else pop 2 elements, combine and push onto stack
        else{
            let operand1 = stack.pop().unwrap();
            let operand2 = stack.pop().unwrap();
            let combine = format!("({} {} {})" ,operand2 , item , operand1 );
            stack.push( combine ); 

        }
        
        
    }
    //setting expression infix to be equal to clone of first element
    self.infix = stack[0].clone();
}

}

/**
 * Parses and handles command line argument, and contains the logic and code to run the program.
 * If there is an error writing to the output file, main should print an appropriate error message.
 */

 fn main(){



    let mut args: Vec<String> = env::args().collect();
    //handles command line arguemnts commented out for testing
   // if args.len() != 3 {
     //   println!("Usage: cargo run --example expr [INPUTFILE] [OUTPUTFILE]");
      //  return;  // acts as exit
    //}

    //Gets the 2nd index of the argument array (which should be the file we're pulling from)

    
    // setting input file to the first commane line argument 
    //commented out for testing  
    //let input_file: &str = &args[1];

    let mut expression_list: Vec<Expression> =  build_expression_list("input.txt").unwrap();//if inside src directory path is input.txt
    
    
/* 
    for i in 0..expression_list.len(){
    solve(&mut expression_list[i]);

    to_string(&expression_list[i]);
    }
*/

   for i in 0..expression_list.len(){
    expression_list[i].solve();
    expression_list[i].postfix_to_infix();
    println!("{}", to_string(&expression_list[i]));
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


fn to_string(expression: &Expression) -> String{
    
   // println!("Postfix:{} \nInfix: {} \nExpr: {:?}", expression.postfix , expression.infix, expression.expr);

   let result = format!("{} = {:?}" , expression.infix , expression.expr);

    


    return result;
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