    // -------------------------------------------------
    // 			Declarative Macros 
    // -------------------------------------------------

/* General Syntax

macro_rules! macro_name {
//   |--- Match rules
    (...) => { ... };
    (...) => { ... };
    (...) => { ... };    // the semicolon at the last rule is optional
}
*/


macro_rules! our_macro {
    () => { 1+1;        
    };

    (something 4 u dear u32 @_@) => {
        println!("You found nonsense here")
    };

    ($e1:expr, $e2:expr) => {
        $e1 + $e2
    };

    ($a:expr, $b:expr; $c:expr) => {
        $a * ($b + $c)
    }
}
fn main() {

    
    println!("{}",our_macro!()); 
    our_macro!(); 
    println!("{}", our_macro!(2,2)); 
    println!("{}", our_macro!(5,6;3));  
    // println!("{}", our_macro!("something",2;"nothing"));  
    
    our_macro!(); 
    our_macro![]; 

    our_macro!{};



    // 1. cargo install cargo-expand
    // 2. rustup install nightly  
    // 3. rustup component add rustfmt
    // 4. rustup component add rustfmt --toolchain nightly  


} 