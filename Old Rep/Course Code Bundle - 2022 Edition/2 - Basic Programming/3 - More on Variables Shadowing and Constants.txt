fn main() {


    // -------------------------------------------------
    // 			Initializing mutliple variables 
    // -------------------------------------------------

   let (first_number, second_number) = (250, 480.32); 
   println!("The first number is {} and the second number is {}", first_number, second_number);

    // -------------------------------------------------
    // 			Readability of large numbers
    // -------------------------------------------------

    let large_number = 1_000_000; 
    println!("The value of the large number is {}", large_number);


    // -------------------------------------------------
    // 			Integer Overflow 
    // -------------------------------------------------
    
    //let overflow_number:u8 = 256; 
    
   
    // -------------------------------------------------
    // 			Decimal numbers in other formats
    // -------------------------------------------------
    

    let x= 255; 
    println!("The value of the variable x in hexademical is {:o} and in octal is {:X} and in binary {:b}",x,x,x);
    

    // -------------------------------------------------
    // 			Snake case convention for variables
    // -------------------------------------------------

    // Snake case convention for variable names 
    let Number = 45; // define as let number:i32 = 45  
	
    // -------------------------------------------------
    // 			Operations on number in 
    // 			different formats
    // -------------------------------------------------

	let n1 = 14; 
	let n2 = 15.6; 			
	let n3 = n2 as i32+ n1;         
	println!("The value of n3 = {}",n3);

    // -------------------------------------------------
    // 			Shadowing 
    // -------------------------------------------------
 	
    // Case 1: simple shadowing
    println!("\n***************************************************** \n");

    //let s = 5; 
    //let s = 5*5; 
    //println!("The value of the variable s = {}",s); 

    // Case2: shadowing with mut. it will work 
    //    let mut s = 5; 
    //  let s = 5*5; 
    //println!("The value of the variable s = {}",s); 

   // Case 3: we can change the type ith shadowing also
    /*let s = 32; 
    println!("The variable s = {} is currently an integer",s);
    let s = 'A'; 
    println!("The variable s = {} is currently a character",s);
    let s = 64.5;     
    println!("The variable s = {} is currently a float",s);
    */

    // Case 4: The shadowing in case of code segments
    let mut s = 65; 

    {
        s = 60; // after this change s to let s = 60 and will change the output to 65
	println!("The value of the variable s inside the inner scope = {}",s); 
    }
    println!("The value of the variable s = {}",s); 

    // -------------------------------------------------
    // 			Constants 
    // -------------------------------------------------

    const MAX_SALARY: u32 = 100_000;
    println!("The value of the constant is {}", MAX_SALARY);

}