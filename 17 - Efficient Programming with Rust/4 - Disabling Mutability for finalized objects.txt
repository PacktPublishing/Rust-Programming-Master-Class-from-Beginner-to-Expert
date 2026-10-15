// -------------------------------------------
//           	- Disabling mutability for finalized objects
// ------------------------------------------- 

/* 
fn main() {
    
    // Method 1: using variable rebinding 
    let mut data = vec![5,6,9,4,3];
    data.sort();
    let data = data;


    // method 2: using code block
    let data = {
    let mut data = vec![5,6,9,4 ,3];
    data.sort();
    data
};

}
*/ 


// Example 2: Here's a pattern for disabling mutability for "finalized" objects, even in mutable owned copies of a thing, preventing misuse. Done by wrapping it in a newtype with a private
// inner value that implements Deref but not DerefMut:

#[derive(Debug, Clone)]
pub struct finalized_config<T>(T);

impl<T> Copy for finalized_config<T> where T: Copy {}

impl<T> std::ops::Deref for finalized_config<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Default)]
struct Config {
     a: usize,
     b: String,
}

impl Config {
    fn new() -> Self {
        Self { a: 0, b: String::from("Hello") }
    }

    fn build(self) -> finalized_config<Config> {
        finalized_config(self)
    }
}



fn main() {
    let mut my_configuration = Config::new();
    my_configuration.a = 6;

    let finalized = my_configuration.build();
    
    let mut finalized_copy = finalized;
    //finalized_copy.a = 666;
}

 