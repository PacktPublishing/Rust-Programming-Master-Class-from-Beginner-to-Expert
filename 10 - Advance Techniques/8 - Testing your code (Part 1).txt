

   // -------------------------------------------
   // 			Testing (Part 1)
   // -------------------------------------------


/* 
#[cfg(test)] 
mod tests {
    #[test]     
    fn it_works() {                 
        assert_eq!(123,123);          
    } 

//[lib]
//doctest = false

    
     
    #[test]
    fn test_2() {
        assert_eq!(true,false)  
    }
    
}
*/
 

 // Example 2 
/* 
struct Circle {
    raduis: f32, 
}

impl Circle {    
    fn area(&self) -> f32 {
        3.14 * (self.raduis * self.raduis) 

    } 

    fn perimeter(&self) ->f32 {
         2.0 * 3.14 * self.raduis 
        
    } 

    fn contains(&self, other: &Circle) -> bool {
        self.raduis > other.raduis
    }

} 

#[cfg(test)] 
mod tests { 
    use super::*;
    #[test]     // functions with the test attribute are called test function. functions inside the test module are called helper functions and are not compiled
    fn larger_circle_should_contain_smaller() {
        let larger_circle = Circle{
            raduis: 5.0, 
        };
        let smaller_circle = Circle{
            raduis: 2.0, 
        };
        assert!(larger_circle.contains(&smaller_circle));
     }

     #[test]
     fn smaller_circle_should_not_contain_larger() {
        let larger_circle = Circle{
            raduis: 5.0, 
        };
        let smaller_circle = Circle{
            raduis: 2.0, 
        };
        assert!(!smaller_circle.contains(&larger_circle));
     }

     #[test]
     fn larger_circle_should_have_greater_area() {
        let larger_circle = Circle{
            raduis: 5.0,
        }; 

        let smaller_circle = Circle{
            raduis: 2.0, 
        };
        assert!( larger_circle.area() > smaller_circle.area());

    }
     
    }
*/ 
    
 // Example 3 
            
    /*
    fn division(dividend: f64, divisor: f64) -> Option<f64> {  
        
        match divisor {
            0. => None, 
            _ => Some(dividend / divisor),
        }
         
    }
    
    #[cfg(test)] 
    mod tests { 
        use super::*;
        #[test]     // functions with the test attribute are called test function. functions inside the test module are called helper functions and are not compiled
        fn test_1(){
            let divident= 10.0; 
            let divisor = 0.0;
            let result = division(divident, divisor);
            
            assert!(result!=None, "This is becuase of division by zero, 
            the divident in this case was {} and the divisor was {} ", 
            divident, divisor);

        }
    } 
     */ 

    // Example 4 

     /*
    struct Person{
        name: String,
        age: i32, 
        salary: i32,
    } 


    impl Person { 
        /*
        fn salary_range(&self) { 
            if self.salary <= 10_000 || self.salary >= 30_000 { 
               panic!("The salary must be lesser than 30,000");
            }
        }
        */

        
         
        fn salary_range(&self) { 
            if self.salary <= 10_000 { 

                panic!("The salary must be greater then 10,000");
            } else if self.salary >=30_000 {
                panic!("The salary must be lesser then 30_000");
            }
        }
        
    }
    
    #[cfg(test)] 
    mod tests { 
        use super::*;
        #[test]     
        //#[should_panic]
        #[should_panic(expected = "The salary must be greater then 10,000")]
        fn out_of_range(){ 
            let some_person = Person{
                name: String::from("Nouman"),
                age: 40,
                salary: 32000   // in first example 12000 in second example          
            };  

            some_person.salary_range();  
        }
         
    } 
 */


    /*
    //EXample #5: test cases with result type  
    fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    
  
        match divisor {
            0. => Err(String::from("Error: Division By Zero")), 
            _ => Ok(dividend / divisor),
        }
    }

    #[cfg(test)] 
    mod tests { 
        use super::*;
        #[test]
        fn test(){       
            assert!(division(15.0,0.0).is_ok()) ;   // another useful function is is_err()
    
        }
}
 
 */
// Example 6 


 
mod tests { 
    use super::*;
    #[test]
    fn test() -> Result<(), String>{ 
        if 2+2 == 4 {
            Ok(())
        }else {
            Err(String::from("The two values are not equal"))
        }
    }
}
 