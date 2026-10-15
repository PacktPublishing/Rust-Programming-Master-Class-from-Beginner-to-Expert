// -------------------------------------------
// 			Stack
//          	- Stack using vec
//          	- Application of Stacks (String Reversal)
// -------------------------------------------

fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let poped_val = stack.pop();
    //     println!("The poped value is {:?}",poped_val);
    poped_val
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        //         println!("Can not add more")
    } else {
        stack.push(item);
        //         println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn main() {
    let input_string = String::from("Welcome to rust");
    let size_stack = input_string.len();
    let mut stack = new_stack(size_stack);
    let mut rev_string = String::new();

    
    for i in input_string.chars() {
        push(&mut stack, i, size_stack);
    }

    
    for i in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }
    println!("The input string is {:?}", input_string);
    println!("The reverse of the string is {:?} ",rev_string);
 
}
