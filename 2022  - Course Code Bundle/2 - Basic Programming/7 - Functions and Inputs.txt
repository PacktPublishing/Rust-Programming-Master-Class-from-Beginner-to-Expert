
fn main() {

    // -------------------------------------------------
    // 			Functions
    //			- Basic function (no inputs/outputs)
    //			- Function with inputs
    //			- Function with variables as inputs
    //			- Functions with inputs and outputs
    //			- Functions with multiple outputs
    //			- Code blocks
    // -------------------------------------------------

basic_fn(); 

function_with_inputs("Nouman",40_000); 

let full_name = "Kamran"; 
let salary_info = 50_000; 
function_with_inputs(full_name,salary_info);




let answer = funtion_with_inputs_outputs(10, 15);
println!("The answer of multiplicatin is {}", answer);





let (multiplication, addition, subtraction) = funtion_with_inputs_multiple_outputs(10,15);  
println!("Multiplication = {}, Addition = {}, Subtraction = {}", multiplcation, addition, subtraction);


// Code blocks 
let full_name = {
    let first_name = "Nouman";
    let last_name = "Azam"; 
    format!("{} {}", first_name,last_name) 
};

println!("My full name is {}",full_name); 



let mut n = String::new();
std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read input.");
let n: f64 = n.trim().parse().expect("invalid input");
println!("{:?}", n);

}

// Basic function (no input, no output)
fn basic_fn() {
    println!("This is a basic function"); 
}



// Function with inputs values
fn function_with_inputs(name: &str, salary: i32){
    println!("The name is {} and the salary is {}", name,salary);
}






// Function with inputs and outputs 
fn funtion_with_inputs_outputs(num1: i32, num2: i32) -> i32{
    num1 * num2 // changing this to an expression by adding a semicolon will return an error 
}







// Function with inputs and multiple outputs 
fn funtion_with_inputs_multiple_outputs(num1: i32, num2: i32) -> (i32, i32,i32) {
    num1 * num2 // changing this to an expression by adding a semicolon will return an error 
}


