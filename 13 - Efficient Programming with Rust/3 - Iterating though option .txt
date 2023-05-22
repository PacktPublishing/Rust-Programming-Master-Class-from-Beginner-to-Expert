// -------------------------------------------
//           	- Iterating through options
// -------------------------------------------

fn main() {
    // Example 1:

     
    let some_product = Some("laptop");
    let mut product_vec = vec!["cellphone", "battery", "charger"];
  
    /* 
    match some_product {
        Some(product) =>  product_vec.push(product), 
        _ => {},
    };

    if let Some(product) = some_product {
        product_vec.push(product);
    }

    product_vec.extend(some_product); //
    println!("{:?}", product_vec);

    */

    // Example 2: for adding it to the iterator use the chain
    /*    
    let products_iter = product_vec.iter().chain(some_product.iter());

    for prod in products_iter {
        print!("{:?} ", prod);
    }
    */ 

    
    // Example 3: filtering out none varaints
    
        // do not comment out this line in subsequent examples 
        let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

        // Solution 1
        /* 
        let mut prod_without_none = Vec::new();
        for p in products {
            if p.is_some() {
                prod_without_none.push(p.unwrap());
            }
        }
        println!("{:?}", prod_without_none);
        */  

        // Solution 2: 
        
        /* 
        let prod_without_none = products.into_iter().
        filter(|x| x.as_ref().is_some()).
        map(|x|x.unwrap()).
        collect::<Vec<&str>>();

        println!("{:?}", products);
        
    */ 
        // The idiomatic way is to use the flatten function
        let products: Vec<&str> = products.into_iter().flatten().collect();
        println!("{:?}", products);


}
