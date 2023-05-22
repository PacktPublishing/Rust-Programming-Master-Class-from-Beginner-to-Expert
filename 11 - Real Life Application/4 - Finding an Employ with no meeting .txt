   // -------------------------------------------
   // 			Employee with no Meeting
   //           	- Description 
   //           	    - Given meeting schedule of employees, we want to determine the 
   //                     overlappign time  
   //                     
   //           	- Tools
   //           	    - MultiDimensional Arrays, Nested Loops
   // -------------------------------------------
 
  
// essential condition is to check for two ranges of the form [x1, x2] and [y1,y2] is max(x1,y1) <= min(x2,y2)
use std::cmp;
fn overlapping_meetings( meetings_a: Vec<Vec<i32>> ,  meetings_b: Vec<Vec<i32>>) ->  Vec<Vec<i32>> {
    let mut intersection: Vec<Vec<i32>> = Vec::new();
    for i in 0..meetings_a.len() {
        for j in 0..meetings_b.len() {
            let (st_a, st_b) = (meetings_a[i][0],meetings_b[j][0]); 
            let (ed_a, ed_b) = (meetings_a[i][1],meetings_b[j][1]); 
            let overlap_status = overlap(st_a, st_b, ed_a, ed_b); 
            if overlap_status != None{
                intersection.push(overlap_status.unwrap());
            }

        }
    }
    intersection
}

fn overlap(start_a:i32, start_b:i32, end_a:i32, end_b:i32) -> Option<Vec<i32>> {
    let mut intersection_time: Vec<i32>= Vec::new();
    if cmp::max(start_a,start_b) < cmp::min(end_a,end_b) {
         intersection_time.push(cmp::max(start_a,start_b)); 
         intersection_time.push(cmp::min(end_a,end_b)); 
         Some(intersection_time)
    } else {
        None
    }
}


fn main() {
    let meetings_sec_a: Vec<Vec<i32>> = vec![vec![13, 15], vec![15, 16], vec![7, 9]];
    let meetings_sec_b: Vec<Vec<i32>> = vec![vec![14, 15], vec![5, 10]];
    
    
    let intersection = overlapping_meetings(meetings_sec_a, meetings_sec_b);
    println!("The overlapping timings are {:?}",intersection);
}
