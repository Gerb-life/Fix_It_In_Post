use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write, Error};
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
                let result;

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
            if !is_operator(item){
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

    fn to_string(& self) -> String{
        // println!("Postfix:{} \nInfix: {} \nExpr: {:?}", expression.postfix , expression.infix, expression.expr);
        let result = format!("{} = {:?}\n" , self.infix , self.expr);
        return result;
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
    
    solve_list(&mut expression_list);
    sort_list(&mut expression_list);
    write_to_file("testOutput.txt", &mut expression_list).expect("Failed to create and write");


    for i in 0..expression_list.len(){
        print!("{}" , expression_list[i].to_string());
    }

}

/**
 * This function accepts a reference to a string slice representing the input file name
 * and returns a ‘Result’ with a vector of expressions from the file or an appropriate error.
 */
fn build_expression_list(input_file: &str) -> Result<Vec<Expression> , Error>{
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
    for i in 0..expression_list.len(){
        expression_list[i].solve();
        expression_list[i].postfix_to_infix();
       } 
}

/**
 * Takes a reference to a vector of Expressions and sorts them based on the value of the
 * expressions solution.
 */
fn sort_list(expression_list: &mut Vec<Expression>){
    // We should pass the Vector to this method after we've pass it to solve_list()
    let length = expression_list.len();

    for i in 0..length{
        let smallest = find_smallest(expression_list, i);   
        expression_list.swap(smallest , i);
    }
}

fn find_smallest(expression_list: &mut Vec<Expression>, current: usize)-> usize{
    let length = expression_list.len(); 
    let mut smallest = current;
        
    //if vector at i is less than smallest then change smallest
    for i in (current +1)..length{
        if expression_list[i].expr[0] < expression_list[smallest].expr[0]{
            smallest = i;
        }
    }  
    smallest
}

/**
* This takes a reference to a string slice, representing the output file name and a reference
 * to a vector of expressions. This function returns a ’Result’
*/

fn write_to_file(fileName: &str , expression_list: &Vec<Expression>) -> Result<(), Error> {
    let mut created = File::create(fileName)?;

    for expr in expression_list{
        let output = expr.to_string();
        created.write(output.as_bytes())?;
    }
    Ok(())
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