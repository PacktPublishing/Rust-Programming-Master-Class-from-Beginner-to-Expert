fn main()
{ 
  // -------------------------------------------
  // 			Match
  // 			- Simple match
  //			- If else ladder into a match
  // 			- If let syntax style match
  // -------------------------------------------
/* General syntax 

   match value {

       possible_value(s) => {Statements to execute},
       possible_value(s) => {Statements to execute},
       possible_value(s) => {Statements to execute},
          
       _ = { default_execution_statements },
   };
*/    
    /*
   let some_number = 100; 
   match some_number {
       1 | 2 => println!("The number is -1"),  
       2 | 3 => println!("The number either 2 or 3"), //    please note a single pipe here for the conditional OR
       4..=100 => println!("The number is between 4 and 100 inclusive"),       
       //_ => println!("The number is greater than 100"), 
   }
   */

    
   /*
   let marks = 50; 
   let mut grade = 'N'; 

   match marks {
   90..=100 => grade = 'A', 
   80..=89  => grade = 'B', 
   70..=79  => grade = 'C',
   60..=69  => grade = 'D',
   _ =>       grade = 'F',
   }
   println!("The grade achieved is {}",grade);
   */
   

/*    
      let variable = match value {
       possible_value(s) = {Statements to execute},
       possible_value(s) = {Statements to execute},
       possible_value(s) = {Statements to execute},
          
       _ = { default_execution_statements }
   };
  */ 

   let marks = 98; 
   let grade = match marks {
   90..=100 =>  'A', 
   80..=89  =>  'B', 
   70..=79  =>  'C',
   60..=69  =>  'D',
   _ =>         'F',
   };
   println!("The grade achieved is {}",grade);

}