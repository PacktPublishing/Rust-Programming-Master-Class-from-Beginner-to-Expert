    
    fn main()
{

    // -------------------------------------------
    // 		References Rules  
    //          - One mutable reference in a scope
    //          - Many immutable references
    //          - Mutable and immutable can not coexist
    //          - Scope of a reference 
    //          - Data should not change when immutable references are in scope 
    // -------------------------------------------

   /*

let mut heap_num = vec![4, 5, 6]; 
let ref1 = &mut heap_num; 
let ref2 = &mut heap_num;  
                                         
println!("The first reference is {:?} and the second one is {:?}",ref1,ref2);


let mut heap_num = vec![4, 5, 6]; 
let ref1 = &heap_num; 
let ref2 = &heap_num; 
println!("The first reference is {:?} and the second one is {:?}",ref1,ref2);




let mut heap_num = vec![4, 5, 6]; 
let ref1 = &heap_num; 
let ref2 = &heap_num; 
let ref3 = &mut heap_num;  
println!("Immutable references are {:?} and {:?} and the mutable reference is {:?}",ref1,ref2,ref3); 
 

let mut heap_num = vec![4, 5, 6]; 
let ref1 = &heap_num; 
let ref2 = &heap_num; 
println!("Immutable references are {:?} and {:?}",ref1,ref2); 
let ref3 = &mut heap_num;   
*/


let mut heap_num = vec![4, 5, 6]; 
heap_num.push(68);

let ref1 = &heap_num; 
let ref2 = &heap_num; 
println!("Immutable references are {:?} and {:?}",ref1,ref2); 
heap_num.push(86);



}


    