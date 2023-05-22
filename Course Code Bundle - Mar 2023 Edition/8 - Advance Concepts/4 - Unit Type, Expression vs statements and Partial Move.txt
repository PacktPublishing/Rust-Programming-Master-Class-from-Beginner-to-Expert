   // -------------------------------------------
   // 			- Unit Type  
   //			- Expression vs statements 
   //       		- Partial Move 
   // -------------------------------------------

   /*
   fn main() {
    let x: () = ();
   }
  */
  
  
  /*
   fn f1() -> () {}
    
   fn f2() {}
   fn main() {
    f1(); 
    f2();   
    }
   */


   /*
   fn main() {
    let x  = {
      println!("Hello and welcome to the code"); 
      println!("This program will compute the taxes"); 
    };
  }
 */
   
 /* 
 fn division(divident:f64, divisor:f64) -> Result<(), String> {
    let answer = match divisor {
      0.0 => {
        println!("Error of division by zero");
        Err(String::from("Error: Division by zero"))},
      _ =>   {
        println!("The division is valid");
        Ok(())} 
      };
      answer
   }
   fn main()  {
   division(9.0, 3.0);
   }
 */


/*
fn main() {
    let x: () = ();
    let y: () = println!("Hello, world!");
    println!("The two variables are equal {}", x == y );
}
*/



/*
fn main() {

    let num = 10;  // statement
    //let num1 = (let x = 100); // this is not correct becuase statements does not return something 

    // expressions inside a statement 
    let y = {
        let x = String::from("Hello Rust"); 
        x
    };

    // clarification on functions 
    // Function call = expression 
    // Function declaration/definition is a statement 

    square(4);  // expression 

    // this is also a statement
    println!("This is a s simple statement"); 

}

fn square(x:i32) ->i32 {
    x*x
}
 */


#[derive(Debug)]
struct Student {
    name: String,
    courses: Vec<String>,
    age: u8
}


fn main() {

    let student_1 = Student {
        name: String::from("Salman"),
        courses: vec!["maths".to_string(), "geography".to_string(), "physics".to_string()], 
        age: 22,
    };

    println!("The person's age is {:?}", student_1); 


    let name =  student_1.name; 
    let courses = &student_1.courses;
    let age = student_1.age;

    //println!("The person's age is {}", student_1.name);
    println!("The person's name is {:?}", student_1.courses);  
    println!("The person's salary is {}", student_1.age); 

  
    println!("The person's age is {:?}", student_1); 
}


 


