    // -------------------------------------------
    // 			Smart Pointers
    //          	- Box Pointers 
    //          	- Issue with Box Pointer implementation of Cons variant in List
    // 			- Custom Defined Smart Pointers 
    // -------------------------------------------

/*#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}",list);
}
*/

//Example #2: 
/*
enum List {
   Cons(i32, Option<Box<List>>),    
 }
 
use List::{Cons};

fn main() {  
   let list = List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))));
}

*/ 



/* 
struct MySmartPointer{value: i32}    // add it at the end 
    impl MySmartPointer {
        fn new(x:i32)-> MySmartPointer{
           MySmartPointer{value: x}
    
        }
    }
    
   use std::ops::Deref; 
   /*
   trait Deref {
    type Target: name_of_type;
    fn deref(&self) -> &Self::Target;
   }  
   */ 

   
   impl Deref for MySmartPointer {
   type Target = i32;
   fn deref(&self) -> &i32 {
      &self.value //returns data
      }
   }
   
    /*
    pub trait Drop {
    fn drop(&mut self);
   }
    */
    
    impl Drop for MySmartPointer{
        fn drop(&mut self){
           println!("dropping MySmartPointer object from memory {:?}", self.value);
        }
    }
    
        
     fn main() {
        
        let a = 50;
        let b = &a;   //let b = Box::new(a); 
        println!("{}", 50 == a);
        println!("{}", 50 == *b); 
        //println!("{}", a == b); 
        
          
        let sptr1 = MySmartPointer::new(a);
        let sptr2 = MySmartPointer::new(a+3);
        let sptr3 = MySmartPointer::new(a+6);
         
        println!("{}", a==*sptr1); // *(sptr1.deref())                         // In the first place explain the deref and then in the second case explain the drop
        let z = *sptr1; 

        println!("The items will be dropped after this line executes");     
        drop(sptr2);   
    }

*/ 


use std::ops::Deref;

struct Courses {
    courses_list: Vec<String>,
    department: String,
    year: u32,
}

impl Deref for Courses {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.courses_list
    }
}

fn main() {
    let current_courses = Courses {
        courses_list: vec!["CS101".to_string(), "CS303".to_string(), "CS400".to_string()],
        department: String::from("CS"),
        year: 2022,
    };

    println!("{:?}", *current_courses);
}


