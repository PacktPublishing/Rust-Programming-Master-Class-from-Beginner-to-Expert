fn main() {

    // --------------------------------------
    // 			Variables in Rust 
    // --------------------------------------

    let mut x = 15;  
    println!("the value of variable x = {}",x); 
    
    x  = 60; 
    println!("the value of variable x = {}",x); 

    let y = 5 *5; 
    

    // --------------------------------------
    // 			Data Types 	
    // --------------------------------------


    // --------------------------------------
    // 			Scalar Data Types
    //	 		- Integers 
    //				- Singed i8, i16, i32, i64, i128	
    //				- Unsigned u8, u16,u32,u64, u128	
    // --------------------------------------


    println!("The Maximum number in i8 = {}", std::i8::MAX);
    println!("The Maximum number in u8 = {}", std::u8::MAX);

    // --------------------------------------
    // 			- Floats 	
    //				- f32, f64
    // --------------------------------------

    let z = 3.65;
    println!("The addition of an integer and float is {}",x+z);
    println!("The Maximum number in f32 = {}", std::f32::MAX);

    // --------------------------------------
    // 			- Boolean  	
    // --------------------------------------

    let status = false; 
    println!("The value of some of our variables are {:?}", (x,y,z,status));

    let not_equals = 18 != 18;   // you can use > < >= or <=
    println!("The value of condition is 18! = 18 is {}", not_equals);

    // --------------------------------------
    // 			- Characters 	
    // 				- Represent single letters, 
    //  			- digit, 
    //				- emoji's or 
    //				- unicode scalar values	
    // --------------------------------------

   

    let c1 = 'a';
    let c2 = '3';
    let c3 = '+';
    let c4 = '\u{288A}'; 
    let c5 = '\"'; 
    println!("The value of c1 is {}, c2 is {}, c3 is {}, c4 is {} and c5 is {}",c1,c2,c3,c4,c5);


}