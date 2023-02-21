// -------------------------------------------
// 			Stack
//          		- Stack using vec
//          		- Application of Stacks (String Reversal)
//          		- Application of Stacks (Expression Evaluation)
// -------------------------------------------

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let poped_val = stack.pop();
    poped_val
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    //(44+33)*(34+39)-36
    for i in input_chars {
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }
    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("{:?}", tokenized_input);
    tokenized_input
}

/*
1. priorities of operators 
	->  +,- 
	->  *,/ 
	->  ^
2. if scanned oeprator is <= then the top of the stack in priority 
then pop opertors until we have low priority. Add the the poped elements 
to the postfix

3. if "(" pust it to the stack 

4. if ")" pop elements until "(" and add poped elements to postfix

5. if operand then just add to the postfix
*/ 

fn priority(x: &String) -> u8 {
    if ("+" == x) | ("-" == x) {
        1
    } else if ("*" == x) | ("/" == x) {
        2
    } else if "^" == x {      
        3
    } else {
        0
    }
}


fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr = input.len();
    let mut stack = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();
    for i in input {
        match i.as_str() {
            // the as_str() does not take ownership as it uses a reference
            "+" | "-" | "/" | "*" | "^" => {

                if size(&stack) == 0 {
                    push(&mut stack, i, size_expr);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        push(&mut stack, i, size_expr);
                    } else {
                        
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            // since the .last returns a reference so not needed to mention &stack.last()
                            postfix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }
                        push(&mut stack, i, size_expr);
                    }
                }
            }

            "(" => push(&mut stack, i, size_expr),
            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }

            _ => postfix.push(i),
        }
    }
    // Check if somethign is left in the stack or not
    while size(&stack) != 0 {
        postfix.push(pop(&mut stack).unwrap());
    }
    println!("{:?}", postfix);


    postfix
    
}

fn operation(op1: String, op2: String, oper: String) -> f32 {
    let op1 = op1.parse::<f32>().unwrap();
    let op2 = op2.parse::<f32>().unwrap();
    let result = match oper.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    };
    result
}
/*
Rules for postfix evaluation 
	1. If operand -> push to stack
	2. If operation pop two selements perform operation and then push into the stack
*/
fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    
    let size_expr = postfix.len();
    let mut result_stack: Vec<String> = new_stack(size_expr);
    for i in postfix {
        match i.as_str() {
            // the as_str() does not take ownership as it uses a reference
            "+" | "-" | "/" | "*" | "^" => {
                let oper = i;
                let op2 = pop(&mut result_stack).unwrap();
                let op1 = pop(&mut result_stack).unwrap();
                let result = operation(op1, op2, oper);
                push(&mut result_stack, result.to_string(), size_expr);
            }
            _ => push(&mut result_stack, i.to_string(), size_expr),
        }
    }
    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}

fn main() {
    let input_expr = String::from("(33+45/3*(2+9)-50)"); // 
    println!("The original Expression is {:?}", input_expr);

    let input_expr_tokenized = individual_symbols(input_expr); 
    let postfix = infix_to_postfix(input_expr_tokenized); 
    println!(
        "The evaluated expressions = {}",
        postfix_evaluation(postfix));
}




