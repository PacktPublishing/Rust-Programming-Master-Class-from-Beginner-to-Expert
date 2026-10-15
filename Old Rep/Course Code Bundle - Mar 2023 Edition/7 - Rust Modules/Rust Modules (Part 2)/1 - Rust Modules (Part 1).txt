/*mod file_1;
 fn main(){

    let rect1 = Rectangle{                   
        length: 5, 
        width:10,
    };
    let area_rect1 = rect_area(&rect1.length, &rect1.width);  // also del the file_1:: if you are including the code in the same script 
}

struct Rectangle{
    length: i32,
    width:i32,
}
*/

/*
fn some_fn() {
    println!("This is the function of the file_1 crate");
} 

mod maths {
    pub mod basic_math {
        pub fn multiplication(num1:&i32, num2:&i32) -> i32{
            let result = num1 * num2; 
            printing(&result);
            result
        }
        fn printing(num: &i32) {
            println!("The result is {}", num);
            crate::some_fn();  
        }
        
    }
}


 fn rect_area(length:&i32, width:&i32) -> i32 {
    // absolute path  
    use maths::basic_math::multiplication;
    multiplication(length, width)  // parents can not see child modules but childs can see parents modules by default 
}
 */


 /*
mod file_2; 
fn main() {
    file_2::some_person(); 
}
 */

mod file_3; 

fn main() 
{
    file_3::allowance();
}
