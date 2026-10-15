// -------------------------------------------
// 		RefCell
// 		    - Borrowing rules checked at run time 
// 		    - Interior Mutablity
// 		    - Rc with RefCell

// -------------------------------------------


/*
use std::cell::RefCell;  

fn main()  
{  

    /*
    let mut x = 50; 
    let x1 = &x; 
    let x2 = &x; 
    let x3 = &mut x;

    println!("{} {} ", x1,x2);
   */

   /* 
    let a = RefCell::new(10);   
 
    { 
    let b = a.borrow();  
    let c = a.borrow(); 
    }

    //drop(b); 
    //drop(c); 
    
    let d  = a.borrow_mut();  
    drop(d);
    println!("Value of a is : {:?}",a);

   */

   /* 
   let x = 32; 
   let x1 = &mut x; 
   */ 


   /*
   let a = RefCell::new(10);   
   // *a.borrow_mut() = 15; 
   let mut b = a.borrow_mut();
   *b = 15;

   //drop(b); 
   println!("{:?}",a);



    */
  //} 

  */




use std::cell::RefCell;  
use std::rc::Rc; 


 
fn main()  
{  


  let a = Rc::new(RefCell::new(String::from("java")));  
  let b  = Rc::clone(&a); 
  
  *b.borrow_mut() = String::from("c++"); 
  println!("{:?}",a); 

  

}  

 