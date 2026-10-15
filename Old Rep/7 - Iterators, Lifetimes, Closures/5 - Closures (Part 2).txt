   // -------------------------------------------
   // 			Closures   
   //           	- A quick recap
   //           	- Borrow by immutable reference
   //           	- Borrow by a mutable reference
   //           	- Moving of a value into a closure
   // -------------------------------------------



/*
fn main(){

let some_closure_1 = |x: u32| -> u32 { x + 1 }; 
let some_closure_2 = |x| { x + 1 };    
let some_closure_3 = |x|  x + 1 ; 
} 
 */



/*
fn main(){
let mut vec_1 = vec![1, 2,3];
let some_closure = || {
    // vec_1 is being used by reference.  
    println!("Vec 1 : {:?}", vec_1);     
};

println!("Vec_1: {:?}",vec_1);              
some_closure();    
                   
vec_1[1] = 15; 
}
*/ 


/*
fn main(){
    let mut vec_1 = vec![4,5,6];
    let mut some_closure = || {
        
        vec_1.push(35); 
       
    };
    
    //println!("vec_2 {:?}", vec_2);  
    //vec_1[1] = 15;                   
    some_closure();    
                        
    
    // vec_1[2] = 15; 
    
    }
    
*/ 
   
    fn main(){
        let mut vec_1  = vec![1,2,3];
        let some_closure = || {
            
            let vec_2 = vec_1;
        };
        

        some_closure();     
        println!("vec 1 = {:?} ", vec_1);          
        println!("vec 2 = {:?} ", vec_2);          
        
    }
        