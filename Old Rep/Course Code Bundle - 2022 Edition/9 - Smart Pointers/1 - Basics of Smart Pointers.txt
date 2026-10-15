// -------------------------------------------
// 		Smart Pointers
//          	- Box Pointers 
// -------------------------------------------


fn main() {
    let single_value = Box::new(0.625); 
    let x = 0.625;              
    println!("Are the values being equal {}", x == *single_value);   // deref is needed when box contains a single value
            
           
    let mut stack_var = 4; 
    let stack_ref = &stack_var; 
    let heap_var = Box::new(stack_var);        // what happens when we write stack_var inside the (stack_ref)
    

    stack_var =  5; 
    println!("The value of stack_var = {} and heap_var = {}", stack_var, heap_var);
    

    let point = Box::new((100, 125)); 
    println!("{} {}", 100 == point.0, point.1);   
    
    let x = point; //     
}