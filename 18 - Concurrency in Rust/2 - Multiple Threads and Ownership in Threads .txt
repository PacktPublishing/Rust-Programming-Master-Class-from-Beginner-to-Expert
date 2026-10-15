   // -------------------------------------------
   // 			Multiple Threads
   // 			Ownership and Threads	 
   // -------------------------------------------

   use std::thread;
   fn main(){

    /* 
    let mut thread_vec = vec![]; 
    for i in 0..10 {
        thread_vec.push(thread::spawn(move || {
            println!("Thread number {}", i);
        }));
    }

    for i in thread_vec {
        i.join();
    }
    */ 

    let v = vec![1,2,3]; 
    let x  = 5; 
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v); 
        println!("Here's a variable : {:?}", x); 
    });

    drop(x);
    println!("The variable x is still alive {}", x) ;
    println!("The variable v is not alive {}", v) ;
    handle.join();
   }
