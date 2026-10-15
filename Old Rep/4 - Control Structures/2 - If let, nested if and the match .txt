fn main()
{
  // -------------------------------------------
  // 			Condition If 
  // 			- Nested If
  // 			- If let 
  // 			- If let (in case of if else ladder)   
  // 			- Match expression 	
  // -------------------------------------------

   /*     
   if outer_condition {
       // Statement to execute if the outer_condition is true 
       if inner_condition {
           // Statements to execute if both the outer and inner conditions are true 
       } else { 
           // some statements to execute
        }

    else {
        // some statements to execute
    }
    }
   */  


   // Example 
   /*
   println!("Enter a Number"); 
   let mut some_num = String::new();
   std::io::stdin()
       .read_line(&mut some_num)
       .expect("failed to read input.");
   let some_num: i32 = some_num.trim().parse().expect("invalid input");
   
   // Outer condition 
   if some_num != 0 {     // !(some_num == 0) this is equal to sum_num != 0
       // inner if condition
       if some_num % 2 == 0 {
           println!("The number is even.");
       } else {  // inner else condition 
           println!("The number is odd.");
       }  
   } else {      // outer else condition
       println!("The number is 0 and it is neither even nor odd.");
   }
   */


   // 2. if let 
   /*
   let variable_name = if condition {
       // Statements to execute and 
       // return value of variable without a semicolon 
   } else {
       // Statements to execute and 
       // value of variable of variable without a semicolon 
   };  // semicolon 
   */ 

    /*   
   let value = if true {
       1
   } else {
       2   // 2.0 will generate an error 
           // please note that the returns from all the statements should be of the same type 
           // The else part must exist otherwise the variable may be empty which the rust wont allow
   }; 
   println!("Value = {}", value); 
   */ 

   let marks = 50; 
   
   let grade = if marks >= 90 {  
       'A' 
   } else if marks >= 80  {
       'B'
   } else if marks >=70 { 
       'C'
   } else if marks >=60  {
       'D'
   } else {             // Deleting the else block will not compile 
       'F' 
   };

   println!("The obtained grade is {:?}",grade);


}


