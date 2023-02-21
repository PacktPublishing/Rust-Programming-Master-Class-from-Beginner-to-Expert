
   fn main()
   {   

       let mut condition = true;
       println!("Please enter the marks in percentage for the students\n"); 

       let mut grades = vec![]; 
       while condition {
           println!("'Please enter the student marks"); 

            let mut marks_input = String::new();                                       
            std::io::stdin()
             .read_line(&mut marks_input)
             .expect("failed to read input.");

            let marks_input: i32 = marks_input.trim().parse().expect("invalid input"); 
            grades.push(marks_input); 
            println!("Do you want to enter another student data [Y/N]"); 

            let choice: char = {
                let mut choice_input = String::new();                                

                std::io::stdin()
                .read_line(&mut choice_input)
                .expect("failed to read input.");

                choice_input.trim().parse().expect("invalid input")
            };
            
           
            // Try using the break instead of this logic 
            if choice_input == 'Y'{
               condition = true;    
           } else { condition = false; }
        
                   
       }
   
       println!("The students grades are = {:?}", grades); 
    }
   
   