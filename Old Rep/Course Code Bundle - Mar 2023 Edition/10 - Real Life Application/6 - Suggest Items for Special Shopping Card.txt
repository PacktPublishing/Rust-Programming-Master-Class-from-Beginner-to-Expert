   // -------------------------------------------
   // 			Suggesting Items for Special Shopping Card 
   //           	- Description 
   //           	    - Given a list of prices, return a couple of items with their sum matching the given price    
   //           	- Tools
   //           	    - Hashsets, Vectors 
   // -------------------------------------------

use std::collections::HashSet;
fn product_suggestions(product_prices:Vec<i32>, amount:i32) -> Vec<Vec<i32>>{
    let mut prices_hash = HashSet::new(); 
    let mut offers = Vec::new(); 

//ec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];
    for i in product_prices {
        let diff = amount-i; 
        if prices_hash.get(&diff).is_none()  { 
            prices_hash.insert(i);
        }  else {
            offers.push(vec![i, diff]);           
        } 

    } 
    
    offers
}




// Driver code
fn main(){
    let product =vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23];

    let suggestions = product_suggestions(product, 50);
    println!("{:?}",suggestions);
    //return 0;
}