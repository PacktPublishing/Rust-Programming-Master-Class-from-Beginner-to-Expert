    // -------------------------------------------
    // 			Rust Ownership 
    // 			- Each value in Rust has a variable that’s called its owner.
    // 			- There can be only one owner at a time.
    // 			- When the owner goes out of scope, the value will be dropped.
    // -------------------------------------------


    fn main() { 
        // -------------------------------------------
        // 			Ownership and functions
        // -------------------------------------------
        
        /*
       let stack_num = 32;  
       let mut heap_num = vec![4, 5, 6]; 
       
       stack_function(stack_num); 
       println!("The stack variable is copied and the orginal value was {}", stack_num);
       
       heap_function(&mut heap_num);  
       println!("The variable after the function call is  {:?}", heap_num);  
       */ 
        // -------------------------------------------
        // 			Quiz
        // -------------------------------------------

        let mut heap_num = vec![4, 5, 6]; 
        let ref1 = heap_num;    
        let ref2 = &ref1; 


        // -------------------------------------------
        // 			A common mistake
        // -------------------------------------------

        let mut heap_num = vec![4, 5, 6]; 
        let mut ref1 = &heap_num;    
        
        println!("Immutable references are {:?} ",ref1); 


        
        // -------------------------------------------
        // 			When will references becomnes handy
        // -------------------------------------------
        let large_data1 = String::from("This is the first long string"); 
        let large_data2 = String::from("This is the second long string"); 
        
        let huge_data:Vec<&str> = vec![&large_data1, &large_data2];  
        println!("The values of the the combined strings are '{:?}", huge_data);
       }
       
       fn stack_function(mut var:i32)    
       {   
           var = 56; 
           println!("The copied value of the variable has been updated to {}", var); 
       }
       
       
       fn heap_function(var:&mut Vec<i32>)    // wihtout & in first example
       {
           var.push(50);    
           println!("The value of the vector inside the function is {:?}",var); 
       }
       
      