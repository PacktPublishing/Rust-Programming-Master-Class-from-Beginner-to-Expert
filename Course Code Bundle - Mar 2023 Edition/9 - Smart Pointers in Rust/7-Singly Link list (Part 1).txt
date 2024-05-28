// -------------------------------------------
// 		Link List (Part 1) 
// -------------------------------------------

#[derive(Debug)]
struct linklist{
   head: pointer, 
}
#[derive(Debug)]
struct Node{
    element: i32, 
    next: pointer,     
  }

type pointer = Option<Box<Node>>; 

fn main(){
   /* 
   
   let list = Node{element: 1, next: None};

   let list = Node{element: 1, next: Some(Box::new(Node {
      element: 2, next: Some(Box::new(Node {
         element:3, next: None
      }))
   }))};

   let list = linklist {head: Node{element: 1, next: None}};
   
   let list = linklist {head: Node { element: 1, next: Some(Box::new(Node {
      element: 2, next: Some(Box::new(Node {
         element: 3, next: None
      }))
   })) }}; 
 */ 

 let list = linklist{head: None};

 let list = linklist{head: Some(Box::new(Node {element: 100, next: (
   Some(Box::new(Node {
      element: 200, next: None
   }))
 )}))}; 

 //println!("{:?}", list.head.unwrap().element);  


 //println!("{:?}", list.head.unwrap().next.unwrap().element);  

 println!("{:?}", list.head);



}
