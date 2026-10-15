fn main(){


    // -------------------------------------------------
    // 			Vectors
    //			- Declaring vectors 
    //	 		- Accessing elements
    //	 		- Printing elements
    //	 		- Updating elements 
    //	 		- Initializing with same values
    //	 		- String and char vectors
    //	 		- Vector slices
    //	 		- Common functions 
    //	 		- Invalid access 
    // -------------------------------------------------




// declaring vectors 
let mut number_vec:Vec<i32> = vec![4,5,6,8,9,10,11,12,15,16,12,10];  

// Printing a specific element
println!("{}",number_vec[0]);

// Printing the whole vector 
println!("{:?}",number_vec);

// Updating a specific element 
number_vec[4] = 5; 
println!("{:?}",number_vec);

// Initiallizing vectors with same elements 
let array_with_same_elements: Vec<i32> = vec![0;10];

// String vectors 
let mut string_array_1: Vec<&str> = vec!["apple","tomato","grapes"];
let string_array_2: Vec<&str> = vec!["Unknown";6]; 
string_array_1[0] = "kamran azam"; 

// Character vectors 
let char_array: Vec<char> = vec!['a','p','p','l','e']; 

// Get subset of an vector without making a copy but by reference
let subset_vec = &number_vec[0..3];  // the concept of borrow 
println!("The subset of values of the array are {:?}",subset_vec); 

// Number of elements in an array 
println!("Elements in the array are {}", number_vec.len());

// Check if the elements are there or not 
let check_index = number_vec.get(100); 
println!("{:?}", check_index);

// Adding elements to the vector using the push 
number_vec.push(30);
number_vec.push(40);
println!("The subset of values of the array are {:?}",number_vec); 

// Removing element from the vector 
number_vec.remove(5); 
println!("The array after removing the element at index 5 {:?}",number_vec);

// Checking for an item inside a vector
println!("The value of 10 exist in the array {}",number_vec.contains(&10)); 
}