// -------------------------------------------
//           	- Avoiding Allocations
// ------------------------------------------- 


use std::mem;
/* 
#[derive(Debug)]
enum Customer {
    new { name: String },
    loyal { name: String },
    rich { name: String },
}

fn promote(user: &mut Customer) {
    use Customer::*;
     
    *user = match user {
        //Customer::new { name } => Customer::loyal { name: name.clone() },        
        Customer::new { name } => Customer::loyal {name: mem::take(name)},
        //Customer::new { name } => Customer::loyal {name: mem::replace(name, String::new())},
        
        //Customer::loyal { name } => Customer::rich { name: name.clone() },
        Customer::loyal { name } => Customer::rich {name: mem::take(name)},
        //Customer::loyal { name } => Customer::rich {name: mem::replace(name, String::new())},
        
        Customer::rich { name } => return,
    } 
}

fn main() {
    let mut customer_1 = Customer::new { name: "micheal".to_string() }; 
    
    promote(&mut customer_1); 

    println!("Cutomer 1 {:?}", customer_1);
}

*/


use std::mem::swap;
fn main() {
    let mut s1 = "Nouman".to_string();
    let mut s2 = "Azam".to_string();
 
    //Swapping of variables
    let temp = s1;
    s1 = s2;
    s2 = temp;
    println!("s1: {:?} s2: {:?}",s1,s2);
    

    //The idiomatic way is the one given below
    swap(&mut s1, &mut s2);
    println!("s1: {:?} s2: {:?}",s1,s2);
}

