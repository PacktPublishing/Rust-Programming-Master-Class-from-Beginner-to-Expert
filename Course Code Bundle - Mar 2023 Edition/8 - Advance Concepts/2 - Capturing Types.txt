    // -------------------------------------------------
    // 			Capturing Types
    // ------------------------------------------------- 
    /* 
    macro_rules! input {
        ($t: ty) => {{ 
            let mut n = String::new(); 
            std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input"); 

            let n: $t = n.trim().parse().expect("invalid input");
            n

        }
        };
    }

    macro_rules! add_as {
        ($a: expr, $b: expr, $typ: ty) => { $a as $typ + $b as $typ }
    }

    macro_rules! some_macro {
        ($var: ident) => {
            $var = $var + 1;
            
        };
    }
    fn main(){
        /* 
        println!("Please enter a floating point number"); 
        let some_input_0 = input!(f32); 
        */ 

         
        // println!("{}", add_as!(15,2.3,f32));
        let mut x = 4;
        some_macro!(x);

    }
    */ 

    macro_rules! create_function {
        ($func_name:ident, $input: ident, $type_input: ty, $type_output: ty) => {

            fn $func_name($input:$type_input) -> $type_output {
                println!("You called {:?}() with the input of {:?}", stringify!($func_name), stringify!($input1)); 
                $input
            }
           
        };
    }

    create_function!(f1,x,i32, i32);
    fn main() {
        //f1(15);
        let y = f1(15);
    }