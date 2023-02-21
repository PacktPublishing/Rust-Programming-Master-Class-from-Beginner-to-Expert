
*/

// Example: set union and intersection 

fn main() 
{
    let mut vec_1: Vec<u32> = vec![5,4,3,6,9]; 
    let mut vec_2: Vec<u32> = vec![5,8,6,4,10,15,20,21]; 

   
    let intersect = intersection(&vec_1, &vec_2); 
    println!("\n\n The intersection of the two sets is {:?}",intersect); 

    let union_set = union(&mut vec_1, &mut vec_2, &intersect); 
    println!("\n\n The union of the set is  {:?} \n\n",union_set); 

   
    /* an alternate way for intersection 
    let vec_1_copy = vec_1.clone(); 
   
    // An alternate way of finding the commong 
    let common: Vec<u32> = vec_1.into_iter().filter(|&x| vec_2.iter().any(|&y| y == x)).collect::<Vec<u32>>(); 
    println!("The common value are {:?}", common);   
    // This will however consume the vec_1 

    
    println!("The un_common value are {:?}", un_common);   
    */
}


fn intersection (first_set:&Vec<u32>, second_set:&Vec<u32>) -> Vec<u32>{
let mut common:Vec<u32> = Vec::new(); 

for i in first_set{
    let val = second_set.iter().find(|&x| x == i); 
    match val { 
        Some(common_val) =>  common.push(*val.unwrap()), 

        None => print!(""), 

    } 
}
common

}


fn union<'a> (first_set:&'a mut Vec<u32>, second_set:&'a mut Vec<u32>, common:&'a Vec<u32>) -> Vec<&'a u32>{
    
    for i in common{
        let position_of_common = first_set.iter().position(|&x| {x == *i});  
        first_set.remove(position_of_common.unwrap()); 

        let position_of_common = second_set.iter().position(|&x| {x == *i});  
        second_set.remove(position_of_common.unwrap());  
    }
    let union_set  = first_set.iter().chain(second_set.iter()).chain(common.iter()).collect::<Vec<_>>(); 
    union_set
    
}
