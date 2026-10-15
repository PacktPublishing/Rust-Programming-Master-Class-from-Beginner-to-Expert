    // -------------------------------------------------
    // 			Some Useful Traits
    // -------------------------------------------------
    use std::cmp::Ordering;

    //#[derive( PartialOrd, PartialEq, Clone)]
    #[derive(  Clone)]
    struct Person {
        name: String,
        age: u8,
        earning: u32, 
        savings: u32,
    }

    
    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.age == other.age
        }
    }
    

    
    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.earning.partial_cmp(&other.earning)
        }
    }
     

    fn main() {
        let bob = Person {
            name: "bob".to_owned(),
            age: 30,
            earning: 30_000, 
            savings: 50_000,
        };


        let mut bob_clone = bob.clone(); 
        
        bob_clone.age = 15; 
        bob_clone.name = "eve".to_owned();
        bob_clone.earning = 40_000; 
        //bob_clone.savings = 60_000;
              
        println!("{}", bob == bob_clone);  
        println!("{}",bob >= bob_clone);
        println!("{}",bob <= bob_clone);


        let alice = Person {
            name: "Bob".to_owned(),
            age: 25,
            earning: 30_000,
            savings: 40_000,

        };

        println!(" {}", alice > bob);
    }

