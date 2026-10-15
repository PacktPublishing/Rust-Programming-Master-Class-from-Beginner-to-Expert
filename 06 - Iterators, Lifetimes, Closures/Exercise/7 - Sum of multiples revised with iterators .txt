// Example: Sum of multiples of 3 and 5 -> refined and reduced version of the code with iterators 

/*
fn main() {
    
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input.");
    let n: u32 = n.trim().parse().expect("invalid input");
    
    let divisible_by_3_5 =  (1..n).into_iter().filter(|&x| x % 3 == 0 || x % 5 ==0 ).collect::<Vec<u32>>();   
    println!("{:?}", divisible_by_3_5);   
    println!("{:?}", divisible_by_3_5.iter().sum::<u32>());  
    
    
} 

