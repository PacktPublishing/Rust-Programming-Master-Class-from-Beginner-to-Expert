    // -------------------------------------------
    // 			Smart Pointers
    //          	- Box Pointers 
    //          	- Use Case of Box Pointers 
    // -------------------------------------------


/* 
#[derive(Debug)]
enum List {
    Cons(i32, List),
    Nil,
}
use List::{Cons, Nil};


/*
enum Conveyance {
    Car(i32), 
    Train(i32),
    Air(i32), 
    Walk
}
*/
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
//    let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    println!("{:?}",list);
}
*/


#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}",list);
}

