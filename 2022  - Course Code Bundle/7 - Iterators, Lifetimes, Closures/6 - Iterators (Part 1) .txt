   // -------------------------------------------
   // 			Iterators     
   //           	- Basics
   //          		- Some useful functions for iterators
   //           	- Common statistics 
   // -------------------------------------------

   fn main() 
   {
    /*   let some_vec = vec![1, 2, 3,4,5,6,7];
       let mut iter = some_vec.iter();

       println!("The iterator : {:?}", iter); 
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
   */

   let a:Vec<u32> = vec![0,1, 2, 4,5,6,9,8,7];
       
   let mut check = a.iter().any(|&x| x > 0);     
   println!("The value of the any fucntion is {}",check);
   
   let check = a.iter().all(|&x| x > 0);      
   println!("The value of the all fucntion is {}",check);
   
   let check = a.iter().find(|&&x| x > 0);     
   println!("The value of the  function find  is {}",check.unwrap());
   
   
   let check = a.iter().position(|&x| x > 4);  
   println!("The value of the  function position is {}",check.unwrap());
   
   let check = a.iter().rposition(|&x| x >4);  
   println!("The value of the function rposition is {}",check.unwrap());
   
   
   let check = a.iter().max();     
   println!("The value of the function max is {}",check.unwrap());
 
   let check = a.iter().min();  
   println!("The value of the function min is {}",check.unwrap());
 
   let check:u32 = a.iter().sum();
   let check:u32 = a.iter().product(); 
   println!(" {:?}", check); 
   
   
   let mut iter = a.iter().rev(); 
   println!("The result of applying the rev fucntion {:?}", iter); 
   println!(" {:?}", a); 
}