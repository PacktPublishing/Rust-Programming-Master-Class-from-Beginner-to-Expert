   // -------------------------------------------
   // 			Items in Range
   //           	- Description 
   //           	    - Give a vector of product prices, search for items in a given price range
   //                     
   //           	- Tools
   //           	    - Binary Search Tree, Box pointer 
   // -------------------------------------------
 


struct BinarySearchTree{
     root: Node
}

#[derive(Clone)]
struct Node {
  val: i32,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>
}

impl Node {
  fn new(value: i32) ->Self { 
    Node {
      val: value,
      left: None,
      right: None
    }
  }

  fn insert(&mut self, value: i32) {
		if value > self.val {
			match self.right {
				None => self.right = Some(Box::new(Node {val:value, left:None, right:None})),
				Some(ref mut node) => node.insert(value)
			}
		}
		else {
			match self.left {
				None => self.left = Some(Box::new(Node {val:value, left:None, right:None})),
				Some(ref mut node) => node.insert(value)
			}
		}
	}
}




fn traversal(node: Option<Box<Node>>, low: i32, high: i32,mut  output: &mut Vec<i32>){
    if !node.is_none() {
        if node.as_ref().unwrap().val >= low && node.as_ref().unwrap().val <= high
            {output.push(node.as_ref().unwrap().val);}
        if node.as_ref().unwrap().val >=low 
            {traversal(node.as_ref().unwrap().left.clone(), low, high,&mut output);}
        if node.as_ref().unwrap().val <= high
            {traversal(node.as_ref().unwrap().right.clone(), low, high, &mut output);}
    }
}

fn productsInRange(root: Node, low: i32, high: i32)-> Vec<i32>{
    let mut output: Vec<i32> = Vec::new();
    traversal(Some(Box::new(root)), low, high,&mut output);
    output
}


fn main() {
    // Driver code
    let product_prices = vec![9,6,14,20,1,30,8,17,5];
    let mut bst = BinarySearchTree {
      root: Node::new(product_prices[0])
    };

    for i in 1..product_prices.len() {
    bst.root.insert(product_prices[i]);
    }


    let result = productsInRange(bst.root, 7, 20);
    println!("{:?}",result);
}


