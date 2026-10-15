   // -------------------------------------------
   // 			Sending Multiple Messages
   // 			Multiple Producers
   // 			Threads and Functions
   // -------------------------------------------

 /* 
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let my_vec = vec![1,2,3,4,5];
        for i in my_vec {
            tx.send(i).unwrap(); 


        }
        
    });

     
    for recieved_vals in rx {
        println!("I recieved the value of {}", recieved_vals);
    }
    

    // you may use iterators in this case also
    /* 
    let recived_vals_vec  = rx.iter().collect::<Vec<i32>>();
    println!("The recieved values are {:?}", recived_vals_vec);
     */ 

}

 */ 

 /* 

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();   // to be written after thread 2 code

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));

        }
    });

    thread::spawn(move || {
        // will get an error here stating that tx has been moved. 
        let my_vec = vec![6, 7, 8, 9, 10];
        for i in my_vec {
            tx1.send(i).unwrap();  // later on change to tx1
            thread::sleep(Duration::from_secs(1));

        }
    });

    for recieved_vals in rx {
        println!("I recieved the value of {}", recieved_vals);
    }
}

*/


// Example 4: Creating threads inside a function 
use std::sync::mpsc;
use std::thread;
use std::time::Duration; 

fn timer(d: i32, tx: mpsc::Sender<i32>) {
    thread::spawn(move || {
        //thread::sleep(Duration::from_millis(d as u64)); 
        println!("{} send!",d); 
        tx.send(d).unwrap();
    });
}


fn main(){
    let (tx, rx) = mpsc::channel(); 
    for i in 0..5  {
        timer(i,tx.clone());
    } 

    drop(tx); // this will be written at the end 

    for recieving_val in rx {
        println!("{} recieved!", recieving_val);
    }
}

 