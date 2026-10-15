// -------------------------------------------
// 		Reference Counting Pointers
// -------------------------------------------
/* 
use std::rc::Rc;
enum List{
   //Cons(i32, Box<List>), 
   Cons(i32, Rc<List>),
   Nil,
}

use crate::List::{Cons, Nil};
fn main(){ 
   /* 
   let a = Cons(1, Box::new(Cons(2, Box::new(Nil)))); 
   let b = Cons(3, Box::new(a)); 
   let c = Cons(4, Box::new(a)); 
   */ 

   let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil))))); 
   println!("Count after creating a = {}", Rc::strong_count(&a));

   {
   let b = Rc::new(Cons(3, Rc::clone(&a))); 
   println!("Count after creating b = {}", Rc::strong_count(&a)); 

   let c = Rc::new(Cons(4, Rc::clone(&a))); 
   println!("Count after creating c = {}", Rc::strong_count(&a));
   }
   println!("Count after code block = {}", Rc::strong_count(&a)); 
}
*/ 


use std::rc::Rc; 

fn make_rc() -> Rc<String> {
 
   let s1 = Rc::new(String::from("Hello")); 
   println!("Count when the pointer is created {}", Rc::strong_count(&s1));  

   let s2 = s1.clone(); 
   println!("Count after the clone is created for the pointer {}", Rc::strong_count(&s1)); 
   s2

} 

fn main(){
   let s2 = make_rc(); 
   println!("Count after function call {}", Rc::strong_count(&s2));
}s