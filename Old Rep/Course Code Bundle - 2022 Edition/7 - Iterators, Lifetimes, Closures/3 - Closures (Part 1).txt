   // -------------------------------------------
   // 			Closures   
   //           	- Basic Syntax 
   //           	- Closure with inputs 
   //           	- Same variable for different closures  
   //           	- Ownership and closures  
   //           	- Inference of the output and inputs
   //           	- Passing closure as a function argument
   // -------------------------------------------



//  |...| { ... } 
/*
fn main(){
    let x  = 5; 
    let square = || println!("\n\n The square of variable x is {} \n\n ",x*x);  
    square(); 
}

*/


/*
fn main(){
        let x  = 5; 
        

        let square = |num:i32| println!("\n\n The square of {} is {} \n\n ",num, num*num); 


        square(x);  

        let y = 15; 
        square(y); 
    }
    
*/


/*
    fn main(){
        let x  = 5; 
        

        let square = |num:i32| println!("\n\n The square is {}",num*num);
        let square = |num:i32| println!("\n\n The square is {} \n\n",num*num*num); 


        square(x);  

        let y = 15; 
        square(y); 
    }
    
*/

/*

fn main() {
 
    let print_user_age = |general_info:String, name: &str, age| println!("{}\n\t{}: {}", general_info, name, age);
    let general_info = String::from("The details are"); 
    let (person_name, person_age) = (String::from("Nouman"), 51);  
    
    print_user_age(general_info, &person_name, person_age); 


    println!("The variable has been moved {}", person_name);   
   
}
 
*/


/*
fn main(){
        
    
    let square = |num|  num*num; 

    let x = 5; 
    square(x); 
    
    let y = 105.5; 
    square(y);      
}
*/




/*
fn division<F: Fn(f32) -> bool>(x: f32, y:f32, f: F)  {
    if f(y) == true 
    { 
        print!("\n\n The division result is {} \n\n" ,x/y);
    } else {
        println!("\n\n Dvision is Not possible \n\n ");
    } 
}

fn main() {
    
    let division_status = |y:f32| { if y != 0.0 {true} else {false} };

    division(5.0, 10.0, division_status); 
    division(54.0, 0.0, division_status); 

}
*/