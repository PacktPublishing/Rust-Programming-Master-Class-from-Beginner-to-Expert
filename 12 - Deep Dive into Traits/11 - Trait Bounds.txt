
    // -------------------------------------------------
    // 			- Trait Bounds 
    // ----------------------------------------------



    fn quadruple(x: i32) -> i32 {
// fn quadruple<T>(x: T) -> T {    
// fn quadruple<T: Double>(x: T) -> T { 
        x.double().double()
    }
    
    trait Double {
        fn double(&self) -> Self;
    }
    impl Double for i32 {
        fn double(&self) -> Self {
            self * 2
        }
    }

    // 2.0 
    /* 
    impl Double for i64 {
        fn double(&self) -> Self {
            self * 2
        }
    }
    */
    
    fn main() {
        println!("quadruple 5_i32 == {}", quadruple(5_i64));
    }
    