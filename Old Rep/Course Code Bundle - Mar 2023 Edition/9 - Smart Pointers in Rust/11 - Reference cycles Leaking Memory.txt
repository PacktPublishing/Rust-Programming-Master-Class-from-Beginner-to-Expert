
// -------------------------------------------
// 		Reference Cycles 
// -------------------------------------------
/* 
use std::cell::RefCell; 
use std::rc::{Rc, Weak}; 
#[derive(Debug)] 
struct Node {
    next: Option<Weak<RefCell<Node>>>, 
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
fn main() {    
    let a = Rc::new(RefCell::new(Node {next: None} )); 
    println!("a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); 

    let b = Rc::new(RefCell::new(Node{next: Some(Rc::downgrade(&a))})); 
    println!("B is created: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));  
    println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node {next: Some(Rc::downgrade(&b))})); 

    (*a).borrow_mut().next = Some(Rc::downgrade(&c)); 

    println!("After creating cycle: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a)); 
    println!("b strong count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b)); 
    println!("c strong count: {:?}, c weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c)); 

    println!("a {:?}", a);


}

*/ 


use std::borrow::Borrow; 
use std::rc::{Rc, Weak}; 
use std::cell::{RefCell, Ref}; 

#[derive(Debug)]
struct Node {
    value: i32, 
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() { 
    let leaf  = Rc::new(Node {
        value: 3, 
        parent: RefCell::new(Weak::new()), 
        children: RefCell::new(vec![]),
    }); 

    let branch = Rc::new(Node {
        value: 5, 
        parent: RefCell::new(Weak::new()), 
        children: RefCell::new(vec![Rc::clone(&leaf)]), 
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

}