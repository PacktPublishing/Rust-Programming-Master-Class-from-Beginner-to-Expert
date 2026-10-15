// -------------------------------------------
// 			Most Recently Used Products
//           	- Description
//           	    - A business is interesting in knowing the products that has been
//                       purchased most recently by a customer.

//           	- Tools
//           	    - Hashmaps + Doubly Link List
// -------------------------------------------

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
struct Node { 
    prod_id: i32,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(elem: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            prod_id: elem,
            prev: None,
            next: None,
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;
#[derive(Default, Debug)]
struct List {
    head: Link,
    tail: Link,
}

impl List {
    fn new() -> List {
        List {
            head: None,
            tail: None,
        }
    }

    // a slightly changed version where the function returns the a link now
    pub fn push_back(&mut self, elem: i32) -> Link {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.tail.clone()
    }

    // a slightly changed version of the remove_head and now in this case it returns the head which is removed
    pub fn remove_front(&mut self) -> Option<Link> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                // this statement basically gives us the new head
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail.take();
                    None
                }
            }
        })
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        // check if the next is empty.
        //  -> if it is then 1 . it is already on the tail => do nothing or
        //                   2.  it the only node => do nothing in both cases

        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match (prev, next) {
            (None, None) => {
                // this is the only node in list, so do nothng
            }
            (Some(_), None) => {
                // this node is already the tail, so do nothing
            }
            (None, Some(next)) => {
                // this node is at the head, so we need to move it to the tail
                node.borrow_mut().next = None;
                next.borrow_mut().prev = None;
                self.head = Some(next.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;

                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
        }
    }
}

#[derive(Debug)]
struct MRP_Item {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    item_list: List,
    size: i32,
    capacity: i32,
}

impl MRP_Item {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            item_list: List::new(),
            size: 0,
            capacity: capacity,
        }
    }

    fn purchased(&mut self, prod_id: i32) {
        // First check if we have the entry for the movie already in the hashmap. If yes move to the tail
        if let Some(node) = self.map.get(&prod_id) {
            self.item_list.move_to_tail(node);
        } else {
            // This means that the movie is not in the hashmap

            if self.size >= self.capacity {
                // if the capacity is exhausted then remove head and insert at the tail
                let prev_head = self.item_list.remove_front().unwrap();
                self.map.remove(&prev_head.unwrap().borrow().prod_id);
            }
            // add node to list head
            let node = self.item_list.push_back(prod_id).unwrap();
            // update hashmap

            self.map.insert(prod_id, node);
            // update size
            self.size += 1;
        }
    }
    fn print(&self) {
        let mut traversal = self.item_list.head.clone();
        while !traversal.is_none() {
            let temp = traversal.clone().unwrap();
            print!("{} ", temp.borrow().prod_id);
            traversal = temp.borrow().next.clone();
        }
        println!("");
    }
}
fn main() {
    let mut items_list = MRP_Item::new(3);

    items_list.purchased(10);
    items_list.print();

    items_list.purchased(15);
    items_list.print();

    items_list.purchased(20);
    items_list.print();

    items_list.purchased(25);
    items_list.print();

    items_list.purchased(20);
    items_list.print();
    /*
    items_list.purchased(25);
    items_list.print();

    items_list.purchased(20);
    items_list.print();

    items_list.purchased(5);
    items_list.print();
     */
}
