   // -------------------------------------------
   // 			Lifetimes  
   //           	- Dangling Reference 
   //           	- Undetermined Lifetimes
   // -------------------------------------------

//    use std::vec;

/*
fn main(){
    let i:&i32;                 
    {                           
                            
        let j = 5;              
        i = &j;                 
    }                           
    println!("The value of i = {}", i);   
}                               
*/


/*
fn main() {

    let some_int = 10;  
    let additional_int = some_fn(some_int);  
    println!("{}", additional_int); 

}

fn some_fn(i: i32) ->  &i32{    
    &i                            
}
*/


/*
fn main(){
    let int1 = 5; 
    let int2 = 10; 
    let result = greater(&int1,&int2);
    
}

fn greater(i:&i32,j:&i32) -> &i32 {
    if i> j {
        i                   
    } else {j          
    }

}
*/ 




fn main(){
let s_1 = "Hello";


let v;
{
    let s_2 = String::from("World");       
    v = some_fn(s_1, s_2.as_str());    
}                                       
println!("\n\n{} \n\n", v);             
}

fn some_fn(first_str: &str, second_str: &str) -> &str {
    first_str           
                        
}

