// take help from the video and script in matlab course 
fn main() {
    
        let mut n = String::new();
        std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
        let n: i32 = n.trim().parse().expect("invalid input");
        
        let numbers = 1..n; 
        let mut multiples_of_3 = vec![0];         //  please note that we can not use arrays in this example becuase the length of the array needs to be known at compile time
        let mut multiples_of_5 = vec![0];  
        
        for i in 1..n {
            if i%3 == 0 { multiples_of_3.push(1); } else {multiples_of_3.push(0);}
            if i%5 == 0 { multiples_of_5.push(1); } else {multiples_of_5.push(0);} 
        } 
        
        //println!("\n\n Multiple of 3 = {:?}", multiples_of_3);  
  
        
        let mut combined_list = vec![0]; 
        for i in 1..n as usize {
            if multiples_of_3[i] == 1 || multiples_of_5[i] == 1 {combined_list.push(1)} else {combined_list.push(0);}
        }

        // println!("{:?}",combined_list); 
        
        let mut values_of_multiples:Vec<i32> = vec![0]; 
        for i in 1..=n {
            values_of_multiples.push(combined_list[i as usize] * i); 
            }
        // values_of_multiples.iter().sum(); 

        println!("\n\n Multiple of 3 and 5 are = {:?}", values_of_multiples);  
        
        println!("\n\n The sum of the multiples are = {:?}\n\n", values_of_multiples.iter().sum::<i32>());  // it is having difficutly in inferring the types so i explicilty provide that 
        

    } 
    
    
    
