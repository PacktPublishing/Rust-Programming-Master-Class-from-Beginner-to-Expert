fn main() {
    
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum =0 ;  
    let mut sum_of_squares = 0; 
    for i in 1..=n { 
        square_of_sum = square_of_sum + i;   
        sum_of_squares = sum_of_squares + i.pow(2);  
    }

    let difference = square_of_sum.pow(2) -  sum_of_squares; 
    // println!("The square of sum is {} and the sum of square is {}", square_of_sum.pow(2), sum_of_squares); 
    println!("The difference of the square_of_sum and sum of Squares for N = {} is {}",n, difference);  
    


} 


