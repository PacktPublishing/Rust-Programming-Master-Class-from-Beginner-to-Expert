    // --------------------------------------
    // 			Program Ouputs
    // 			Comments and its Different Styles
    // --------------------------------------

fn main() {
    // This is our first program in this course 
    // This is the second line of comment

    /* this is 
    a 
    multiple line 
    comment 
    */
   
   
    // This is a printing function
    println!("Hello from rust program");

   // comments inside a command
    print/*ln*/!("Hello, world!");
	
    // learning some basic output commands
    println!("The value of the constant is {}",10);

    // Learning to print strings 
    println!("My first name is {} and my last name is {}","Nouman", "Azam");

    // Learning the print command 
    print!("This is a print command "); 
    print!("This is going to be printed on the same line");

    // Learing to write on multiple lines
    print!("\nThis is going to be
            Printed on multiple 
            line"); 
        
    // Learning the use of escape sequences 
    println!("\\n\n This is going to be printed after two lines. \t This will have a tab");

    // Learning somes uses of back slash.
    println!("This will print single quote \' and this double quotes \"");
    println!("This is going to print one back slash \\") ;
    print!("This is some text which will be overwritten \r this text will only appear on the screen");

    // Learning Positional Arguments
    println!("\nI doing {2} from {1} years and i {0} it","like",20,"programming"); 

    // learning Named Arguments 
    println!("{language} is a system progrmaming language which is cool to {activity} in.", language="Rust", activity = "code");
    
    // learning to print basci maths 
    println!("The summation of 25 + 10 = {}", 25 +10);

}