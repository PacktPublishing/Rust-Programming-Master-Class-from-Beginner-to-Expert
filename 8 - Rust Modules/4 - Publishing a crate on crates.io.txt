/* 
these lines are not going to be part of documentation
// documentation comments starts wtih three backslashes. Before publishing something, we need to 
// make sure that our code is error free and contains enough of documentation  


// command for creating the documentation cargo doc --open 
*/



// The lines below ill be part of documentation 
// _______________________________________________

//! # my_proj crate 
//! 
//! This is a collection of some generally used math functions

/// Computes a square of the input number
/// 
/// # Examples
/// ```  
/// let n = 5;
/// let answer = my_proj::square(n);
/// assert_eq!(25,answer); 
/// ``` 
/// # panics
/// some description here
/// 
/// # some other section 
/// Something in this section also 


pub fn square(num:i32) -> i32 
{
    num*num
} 


/// Computes a cube of the input number
/// 
/// # Examples
/// ``` 
/// let n = 5;
/// let answer = my_proj::cube(n);
/// assert_eq!(125,answer); 
/// ```  
pub fn cube(num:i32) -> i32 {
    num*num*num
}


// Steps
// 1. Create login in the git hub and login into the crates.io 
// 2. in the crates.io go to the dashboard and then api tokens and then create new token
// 3. copy the token 
// 4. on the terminal write cargo login and then the long key provided by crates.io 
// 5. go to crates.io again and verify email by going into account setting -> profile -> email -> save 
// 6. make sure that your cargo.toml has following entries  version = "0.1.1", edition = "2021", authors = ["Nouman"], description = "Basic math operations", license = "MIT"
// 7. run the command of cargo publish --allow-dirty 
// 8. to add a revised copy, just change the version number in the cargo.toml and run the command again 
// 9. under your dashboard you will be able to see the crate. 
// 10. inside your dashboard you can yank or unyank the version for stopping it from being downloaded 
// 11. your crate will also be searchable 


/* to explain the usage. make a new project and then use the following crate  
use my_learning::square;
fn main() {
	println!("Hello {}", square(5));
	println!("Hello {}", square(5));

}
*/ 





