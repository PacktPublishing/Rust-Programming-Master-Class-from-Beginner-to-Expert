// -------------------------------------------
// 		Doubly Link list
// -------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

struct List<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    element: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            element: element,
            prev: None,
            next: None,
        }))
    }
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_head = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove_front(&mut self) {
        if self.head.is_none() {
            println!("list is empty so we can not remove");
        } else {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail.take(); 
                        println!("List is empty after removal ");
                        None
                    }
                }
            });
        }
    }




    fn remove_back(&mut self) {
        if self.tail.is_none() {
            println!("list is emtpy so we can not remove");
        } else {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head.take(); 
                        println!("List is empty after removal");
                        None
                    }
                }
            });
        }
    }

   fn print(&self) {
      if self.head.is_none() {
         println!("[]");
         return;
      } else {
         let mut traversal = self.head.clone();
         while !traversal.is_none() {
            print!("{} ", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();

         }  
         println!(); 
      }
   }
}

fn main() {

   let mut list1: List<i32> = List::new();

   list1.remove_front();
   list1.push_front(32); 
   list1.print();

   list1.push_front(23); 
   list1.print();

   list1.remove_front(); 
   list1.print();

   list1.push_back(56); 
   list1.print();

   list1.remove_back(); 
   list1.print();


}
