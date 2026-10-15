
   fn main()
   {   
    // -------------------------------------------
    // 			Loops
    // 			- For loop 
    //          	- Looping through elements using an immutable reference
    //          	- Looping through elements using a mutable reference
    // -------------------------------------------
  
       /*
       let mut some_vec = vec![45, 30, 85, 90, 41,39];  
       
       for i in 0..=5  {    // 0..=5
               println!("The {}th value in the vector is {}", i, some_vec[i]);
       }
       */ 
   
      /*
      let some_vec = vec![45, 30, 85, 90, 41,39]; 
      for i in some_vec {
           println!("{}", i);   
      }      
      println!("{:?}", some_vec);   
      */
      
      /*
      let some_vec = vec![45, 30, 85, 90, 41,39]; 
      for i in some_vec.iter() {    // &some_vec
       println!("{}", i);    
      }      
      println!("{:?}", some_vec);  
       
      */
   
      let mut some_vec = vec![45, 30, 85, 90, 41,39]; 
      for i in &mut some_vec.iter_mut() {    // or &mut some_vec
            *i += 5; 
            println!("{}", i);    
      }      
      println!("{:?}", some_vec);    
   
    }
   
   