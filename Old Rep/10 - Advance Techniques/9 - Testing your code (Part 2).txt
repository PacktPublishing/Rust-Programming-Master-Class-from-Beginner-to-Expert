
   // -------------------------------------------
   // 			How to Run Tests in Different Ways  
   // 			- Coomand line option for tests  
   // -------------------------------------------


// cargo test -- --show-output
/*
fn square(x: i32) -> i32 {
    println!("The square of the number is {}",x*x); 
    x*x
} 

#[cfg(test)]
mod tests{ 
    use super::*; 
    #[test] 
    fn this_passes() {              // change the name in the second example
        assert_eq!(4,square(2));
    }

    #[test] 
    fn this_fails() {           // change the name in the second example 
        assert_ne!(9,square(3));   // change ne to eq in the second example 
    }
}

*/

// Running a specific test:  cargo test square_of_two 
// Subset of test:   cargo run square 
// ignoring some test  cargo test 
// running ignored test: cargo  test -- --ignored

// ignoring test:  
 
/* 
fn square(x: i32) -> i32 {
    println!("The square of the number is {}",x*x); 
    x*x
} 

#[cfg(test)]
mod tests{ 
    use super::*; 
    #[test] 
    fn square_of_two() {
        assert_eq!(4,square(2));
    }

    #[test] 
    fn square_of_three() {
        assert_eq!(9,square(3));
    }

    #[test]
    fn square_of_four() {
        assert_eq!(16,square(4));
     } 

    
     #[test]
     #[ignore] 
     fn huge_test() {
       
       
     }

}
*/
  

pub fn square(x:i32) -> i32 {
    compute_square(x)
} 
fn compute_square(x: i32) -> i32 {
    println!("The square of the number is {}",x*x); 
    x*x
} 


#[cfg(test)]
mod tests{ 
    use super::*; 
    #[test] 
    fn internal_function() {
        assert_eq!(compute_square(2),square(2));
    }      
}
 