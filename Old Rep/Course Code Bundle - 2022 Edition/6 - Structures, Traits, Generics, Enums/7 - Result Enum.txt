   // -------------------------------------------
   // 			Result enum 
   //           	- Basic Syntax and usage
   // -------------------------------------------

   /*
   enum Result<T, E> {
    Ok(T),
    Err(E),
    }
   */ 

// Example 1 
/*
fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    
    /*if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(dividend / divisor)
    }
    */

    match divisor {
        0. => Err(String::from("Error: Division By Zero")), 
        _ => Ok(dividend / divisor),
    }
     
}


fn main() {

    println!("\n\n{:?}", division(9.0, 3.0));  
    println!("{:?}", division(4.0, 0.0));
    println!("{:?} \n\n", division(0.0, 2.0));
} 

*/


// Example 2 

fn main() {
    let some_vec:Vec<i32> = vec![5,5,2,1,5,9]; 
    
    let result1 = match some_vec.get(15) {
        Some(a) => Ok(a), 
        None => Err("The value does not exist"), 
    };

  println!("The value of Result is {:?}", result1);



}
 