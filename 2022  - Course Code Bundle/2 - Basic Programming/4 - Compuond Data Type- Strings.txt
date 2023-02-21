fn main() {


    // -------------------------------------------------
    // 			Strings
    // 			 - Fixed length strings (&str)
    // -------------------------------------------------

let some_string = "Fixed length string"; 
println!("The text inside the string is \"{}\" ",some_string); 


    // -------------------------------------------------
    // 			Strings
    // 			 - Variable length strings 
    // -------------------------------------------------

let mut growable_string = String::from("This string will grow");  
println!("The text inside the string is \"{}\" ",growable_string); 

    // -------------------------------------------------
    // 			Strings
    // 			 - Adding removing characters/strings 
    // -------------------------------------------------

growable_string.push('s');
println!("hey The text inside the string is \"{}\" ",growable_string); 

// Removing one last character 
growable_string.pop(); 
println!("The text inside the string is \"{}\" ",growable_string); 

// Adding some string to the string 
growable_string.push_str(" which i will use");
println!("The text inside the string is \"{}\" ",growable_string); 

    // -------------------------------------------------
    // 			Strings
    // 			 - Functions on strings 
    // -------------------------------------------------

println!("I am going to tell you some basic thing about the string,
Is the string emtpy {}, 
The length of the string is {},
The string has {} bytes, 
Does the string contains the word 'use' {},",growable_string.is_empty(),
growable_string.len(),  
growable_string.capacity(),
growable_string.contains("use"));

growable_string.push_str("   "); 
println!("Length of the string before trim is {},
Length of the string after the trim is {}",growable_string.len(), growable_string.trim().len()); 


// Number to string 
let number = 6;
println!("The value of number in string is {}", number.to_string()); 
println!("Is the number really a string {}", number.to_string() == "6"); 

// chars to string
let some_char = 'a'; 
print!("The value of character in string is {}, 
Is the character really a string {}",some_char.to_string(),some_char().to_string == "a");  

let my_name = "nouman azam".to_string(); 
println!("This string contains my name {}", my_name);


    // -------------------------------------------------
    // 			Strings
    // 			 - Creating an empty string
    // -------------------------------------------------

let empty_string = String::new();
println!("length is {}",empty_string.len()); 

    // -------------------------------------------------
    // 			Strings
    // 			 - Formatting and concatenating strings 
    // -------------------------------------------------

let s_1 = "Nouman".to_string();
let s_2 = "Azam".to_string();
let s_3 = format!("My first name is {}, and my last name is {}",s_1,s_2);
println!("{}",s_3);

// Concatenating strings 
let string_1 = String::from("Nouman");
let string_2 = String::from("Azam"); 
let string_3 = format!("{}{}", string_1,string_2);
println!("The concatenated string is {}",string_3);


}



