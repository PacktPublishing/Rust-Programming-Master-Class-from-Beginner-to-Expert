fn main() { 
    
    let input = String::from("abbbbaa");
    let mut is_palindrome = true; 
    if input.len() == 0 { is_palindrome = true; 
        println!("\n\n The input is palindrome {:?}",is_palindrome);
    return}
    
    let mut last = input.len() - 1;
    let mut first = 0;

    let my_vec = input.as_bytes();

    while first < last {
        if my_vec[first] != my_vec[last] {
            is_palindrome = false;
            break;  // what will happen if we insert a return here 
        }

        first +=1;
        last -=1;
    }

    println!("\n\n The input is palindrome {:?}",is_palindrome);
}
