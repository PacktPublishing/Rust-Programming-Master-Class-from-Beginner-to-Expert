    // -------------------------------------------
    // 			Smart Pointers
    //           	- Custom Defined Smart pointers
    //            	- Deref Coercion  
    // -------------------------------------------

    struct MySmartPointer<T: std::fmt::Debug>{value: T, 
    name: String}    // add it at the end 


    impl<T: std::fmt::Debug> MySmartPointer<T> {
        fn new(x:T)-> MySmartPointer<T> {
           MySmartPointer{value: x, name:String::from("Hello")}
    
        }
    }
    
    use std::ops::Deref; 
    impl<T: std::fmt::Debug> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
      &self.value 
      }
    }
    
    
    
    impl<T: std::fmt::Debug> Drop for MySmartPointer<T>{
        fn drop(&mut self){
           println!("dropping MySmartPointer object from memory {:?}", self.value);
        }
    }
    

    fn my_fn(str: &str) 

    {
       println!("The string recieved from teh main is \"{}\"", str); 
    }
     
 
    #[derive(Debug)]
   struct Person{
   name: String, 
   
}


 fn main() { 
   let sptr_p1 = MySmartPointer::new("Nouman Azam"); 
   my_fn(&sptr_p1);

   let some_vec = MySmartPointer::new(vec![1,2,3]);
   
   for z in &*some_vec { 
       println!("The value is {}", z);

   }   

 }


