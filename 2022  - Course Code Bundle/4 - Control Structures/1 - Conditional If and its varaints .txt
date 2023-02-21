fn main()
{
 
   // -------------------------------------------
   // 			Condition If 
   // 			- Simple If 
   // 			- If with multiple conditions
   // 			- If else 
   // 			- If else if ladder 
   // -------------------------------------------

   // 1. Simple if statement 
   /* General Syntax 

   if condition {
       // statements to execute if condition proves true
   }
   */ 

   // Example 
   /*
    let some_number = 40; 
    if some_number < 50 {
        println!("The number is less than 40"); 
    } 
    println!("This line will execute irrespective of the condition above"); 
  */ 
    
   /* 
   let marks  = 65; 
   if marks >= 60 && marks <= 70 {
       println!("The grade is satisfactory"); 
   }
   */ 

   
   /*
   let flag_1 = true; 
   let flag_2 = false; 
    
   if flag_1 == true || flag_2 == true {     
   
       println!("One of the condition is true here"); 
   }
   

    let flag_1 = true;  
    if flag_1 != false {     
       println!("This will execute when flag_2 is true or in other words 
       when flag_2 is not false"); 
    }
   */ 

   /*
   let flag_1 = true; 
   let flag_2 = false; 
   let number = 60; 

   // flag_1 = true && (flag_2 = false || number <50) and 
   // (flag_1 = true && flag_2 = false) || number <50

   if flag_1 == true && flag_2 == false || number < 50 {
       println!("This part will execute based on the condition above"); 
   }
   */

   // 3. If else conditions 
   /* General syntax 

   if condition {
       // Statements to execute if the condition is ture 
   }
   else {
       // Statements to execite in case the condition is false
   } 
   */ 

   /*
   let marks = 80; 
   if marks > 50 {
       println!("You have passed the exam"); 
   } else {
       println!("You have failed the exam"); 
   } 
   */

   
   /* General syntax 

   if condition {
       // Statements to execute if condition is true 
   }
   else if condition_2 {
       // Statements to execute if condition is true 
   }
   else if condition_3 { 
       // Statements to execute if condition is true 
   }
   else   // optional
   {
       // Statements to execute if all conditions are not true
   }
   */

   
   let marks = 95; 
   let mut grade = 'N'; 
   if marks >= 90 {  
       grade = 'A'; 
   } else if marks >= 80  {
       grade = 'B';
   } else if marks >=70 { 
       grade = 'C';
   } else if marks >=60  {
       grade = 'D';
   } else {             // Deleting the else block will still compile however we may have not condition being true
       grade = 'F'; 
   }

   println!("The obtained grade is {}",grade);
   
}
