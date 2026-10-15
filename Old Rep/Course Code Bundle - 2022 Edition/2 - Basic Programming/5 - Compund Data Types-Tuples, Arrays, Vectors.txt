fn main() {

    // -------------------------------------------------
    // 			Tuples
    //			- Declaring tuples
    //	 		- Destructing tuples
    //	 		- Nested tuples
    //	 		- Emtpy tuples
    // -------------------------------------------------


let my_information= ("Salary", 40_000);
println!("{} is equal to {}",my_information.0,my_information.1); 
println!("Another way of printing the whole tuple is {:?}", my_information);


let (salary, salary_value ) = my_information;
println!("The values of the individual variables are, 
{} and {}",salary, salary_value ); 

let salary = my_information.0;
let salary_value = my_information.1; 
println!("The values of the individual variables are, 
{} and {}",salary, salary_value );

let nested_tuple = (4, 5.0, (3, 2), "Hello");  
let element = nested_tuple.2.0;   // or (nested_tuple.2).0, in some earlier version the syntax will be (nested_tuple.2).0
println!("The value of element is {}", element); 

  
empty_tuple  = (); 


    // -------------------------------------------------
    // 			Arrays
    //			- Declaring arrays
    //	 		- Accessing elements
    //	 		- Printing elements
    //	 		- Updating elements 
    //	 		- Initializing with same values
    //	 		- String and char arrays
    //	 		- Array slices
    //	 		- Accessing elements
    //	 		- Common functions 
    //	 		- Invalid access 
    // -------------------------------------------------

let mut number_array: [i32; 5] = [4,5,6,8,9];  

// Printing a specific element
println!("{}",number_array[0]);

// Printing the whole array 
println!("{:?}",number_array);

// Updating a specific number 
number_array[4] = 5; 
println!("{:?}",number_array);

// Initiallizing arrays with same elements 
let array_with_same_elements = [0;10];

// String arrays 
let mut string_array_1 = ["apple","tomato","grapes"];
let string_array_2 = ["Unknown";6]; 
string_array_1[0] = "kamran azam"; 

// Character Arrays 
let char_array = ['a','p','p','l','e']; 

// array slices
let mut number_array_1: [i32; 5] = [4,5,6,8,9];
let subset_array = &number_array_1[0..3];    // [0..=3]  
println!("The subset of values of the array are {:?}",subset_array); 

// Number of elements in an array 
println!("Elements in the array are {}", number_array_1.len());

// Number of bytes the array is occupying 
/* if we change the type of numbers from i32 to 
i64 then it will occupy more bytes */ 
println!("The array is occupying {} bytes", std::mem::size_of_val(&number_array_1)); 

// Invalid access 
// number_array_1[10] = 5; 

// Check if the elements exists
let check_index = number_array_1.get(100); 
println!("{:?}", check_index);


}
