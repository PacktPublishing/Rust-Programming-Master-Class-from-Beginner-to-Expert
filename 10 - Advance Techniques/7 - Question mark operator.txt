   // -------------------------------------------
   // 			Question Marks Operator   
   // -------------------------------------------

// it will unwrap the value and will only return if the value is ok 
// convenient way for unwrapping the ok variant



/*
use std::num::ParseIntError;
fn parse_str(input: &str) -> Result<i32, ParseIntError> { 

    let integer = input.parse::<i32>()?;   

    println!("The value {:?} is integer {:?}",input, integer);
    Ok(integer)
}
fn main(){
    let some_values = vec!["123", "some1", "some(123)", "abc", "53"];
    for value in some_values{
        println!("{:?}", parse_str(value));  
    }

}
*/





/* 

fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
//fn division(dividend: f64, divisor: f64) -> Result<f64, u8>  {
    
    let answer = match divisor {
    0.0 =>  Err(String::from("Error: Division by zero")),// Err(8),
    _ =>   Ok(dividend / divisor), 
    };

    let correct = answer?;      // it is also going to unwrap the result
    // A small note: this operator will take the ownership
    //println!("{:?}",answer);
    println!("This line will not print in case of error {:?}", correct);
    Ok(correct)
}

fn main() {

    println!("Call from main with result equals to {:?}\n", division(9.0, 3.0));  
    println!("Call from main with result equals to {:?}\n", division(4.0, 0.0));
    println!("Call from main with result equals to {:?}\n",division(0.0, 2.0));
} 

*/
 

/*
 
fn division(dividend: f64, divisor: f64) -> Option<f64>  {
    
    let answer = match divisor {
    0.0 => None,
    _ =>   Some(dividend / divisor), 
    };

    let correct = answer?;      // it is also going to unwrap the result
    // A small note: this operator will take the ownership
    // println!("{:?}",answer);
    println!("This line will not print in case of error {:?}", correct);
    Some(correct)
}

fn main() {
    //let x = division(1.0,2.0);
    println!("Call from main with result equals to {:?}\n", division(9.0, 3.0));  
    println!("Call from main with result equals to {:?}\n", division(4.0, 0.0));
    println!("Call from main with result equals to {:?}\n",division(0.0, 2.0));
} 
 */

/*
#[derive(Debug)]
enum MathError {
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogarithm,
    SqrtError_NegativeSquareRoot,
}

type MathResult = Result<(), MathError>;



    fn division(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionError_DivisionByZero)
        } else {
            println!("The division is successful and has a result of {}", x/y);
            Ok(())
        } 
    } 
    
    
    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::SqrtError_NegativeSquareRoot)
        } else {
            println!("The square root is successful and has a result of {}", x.sqrt());
            Ok(())
        }
    }


    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::LogError_NonPositiveLogarithm)
        } else {
            println!("The log was successful and has a result of {}", x.ln());
            Ok(())
        }
    }
    fn operations(x: f64, y: f64) -> MathResult {  
        division(x,y)?;
        sqrt(x)?;
        ln(x)?; 
        Ok(())
        
        
    }

    fn main() {
         let result = operations(0.0, 10.0);  
         if result.is_ok() {
            println!("All the functions executed successfully"); 
         } else {
             println!("{:?}", result);
         }
    }
 */


// Another example not in the video lectures but simple to understand and very useful
// It will return the current path of the file 
use std::env; 

fn current_dir1() -> std::io::Result<()> {
    let path = env::current_dir()?; // for unwrapping the current path
    println!("\n\n The current path is {:?}\n\n ", path); 
Ok(())
}

fn main() { 
    current_dir1();
    
}




