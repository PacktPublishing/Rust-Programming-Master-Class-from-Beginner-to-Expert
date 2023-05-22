
   // -------------------------------------------
   // 			Fucntion Inputs and Coercion
   // -------------------------------------------

// Example 1
/* 

fn vowels(word: &String) -> u8 {
//fn vowels(word: &str) -> u8 {


    let vowels_count = word_chars.
    into_iter().
    filter(|x|  (*x  == 'a') |(*x  == 'e') |(*x  == 'i')|(*x  == 'o') | (*x  == 'u')).count();
    vowels_count as u8
}

fn main() {
    let affan = "affan".to_string();
    println!("{}: {:?}", affan, vowels(&affan));
    

   println!("Ferris: {}", vowels("Ferris"));

}

*/ 


// Example 2:  &T over &Box<T> becuase it is being coerced to that type 

/* 
//fn length_str(x: &Box<&str>) {
fn length_str(x: &str) {
    println!("length of the string {} is {} ",x, x.len()); 
}


fn main() {
    length_str("hello rust");
    let box_str = Box::new("Hello");
    length_str(&box_str);
}

*/ 



// Example 3:  &[T] over &Vec<T>
//fn square_values(num_vec: &Vec<i32>) {
fn square_values(num_vec: &[i32]) {
        for i in num_vec{
            println!("The square is {}", i*i);
        }
    }
    
    fn main() {
        let values_vec = vec![1,2,3,6,5]; 
        let values_array = [1,2,3,4,5,6];
   
        square_values(&values_vec); 
        square_values(&values_array); 
    }
    