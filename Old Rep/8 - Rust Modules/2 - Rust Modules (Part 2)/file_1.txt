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
            crate::file_1::some_fn();  
        }
        
    }
}


// Point: The program will not compile if we remove the pub word 
 pub fn rect_area(length:&i32, width:&i32) -> i32 {
    // absolute path  
    use maths::basic_math::multiplication;
    multiplication(length, width)  // parents can not see child modules but childs can see parents modules by default 
}

/*
file_1 
        - math 
            - basic maths 
        rect_area()
        some_fn()

 */






