   // -------------------------------------------
   // 			Longest Non-Stop Work
   //           	- Description 
   //           	    - Given time slots numbers, we want to determine the longest consective 
   //                     time slots.
   //                     
   //           	- Tools
   //           	    - HashSet, Vectors, Loops
   // -------------------------------------------
 
use std::collections::HashSet;
fn longest_busy_time(working_slots:  Vec<Vec<u8>>) -> u8{ 
    let mut employee_longest_nonstop_work: Vec<u8> = Vec::new(); 
    for i in working_slots{
        employee_longest_nonstop_work.push(longest_period(i));

    }     

    for i in 0..employee_longest_nonstop_work.len(){
    println!("Employee number {} has worked nonstop for {} slots",i, employee_longest_nonstop_work[i] );
    }


    let max_val = employee_longest_nonstop_work.iter().max(); 
    employee_longest_nonstop_work.iter().position(|x| *x == *max_val.unwrap()).unwrap() as u8
    
}

fn longest_period(working_slots:Vec<u8>) -> u8{ 
    let mut longest_busy_period = 0;
    let  slot_set: HashSet<_> = working_slots.into_iter().collect(); 
    
    for slot in &slot_set{ 
         
        if !slot_set.contains(&(slot - 1)) {
            let mut current_slot = slot.to_owned(); 
            let mut current_consecutive_slot = 1;
            while slot_set.contains(&(current_slot + 1)) {
                current_slot+=1;
                current_consecutive_slot+=1;
            }
            if current_consecutive_slot > longest_busy_period
            {
                longest_busy_period = current_consecutive_slot;
            }
        }
         
    }
    
    return longest_busy_period;
}


fn main() {
    let schedule: Vec<Vec<u8>> = vec![  vec![4, 1, 2, 5, 6, 8, 10, 11], 
                                   vec![3, 1, 2, 5, 7, 10, 11, 14], 
                                   vec![3, 1, 15, 5, 13, 12, 10, 14,15,16,17,18,8, 9]];

    
    println!("Employee number: {} has the highest number of nonstop working slots",
    longest_busy_time(schedule)) ;
}