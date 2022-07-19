fn main()
{   
    // -------------------------------------------
    // 		String concatenation and onwership
    // -------------------------------------------

/*  let s1 = String::from("hello"); 
    let s2: &str = "world";
    
    let s3 = s1 + s2;  // the ownership changed here
    println!("{}", s3);
*/

/*  let s1 = String::from("hello"); 
    let s2 = String::from("world");
    
    let s3 = s1 + &s2;  // The ownership of only owned_string1 changed
    println!("{} {} ", s3 , s2); 
*/

    let s1 = String::from("hello"); 
    let s2 = String::from("world");
    let s3 = String::from(" from Rust");

    let s4 = s1 + &s2 + &s3;  // The ownership of only s1 changed
    println!("{} {} {}", s4,s2,s3); 

}

