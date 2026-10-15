   // -------------------------------------------
   // 			Message Passing through Channels 
   // -------------------------------------------
   use std::thread; 
   use std::sync::mpsc;
   fn main(){
    let (tx, rx) = mpsc::channel();

//    let rx1 = rx; 

    let t = thread::spawn(move || {
        let val = String::from("some data from sender"); 
        println!("Value sending from the thread"); 
        tx.send(val).unwrap();
       // println!("This may execute after the statement in the main"); 
       //  println!("Val is {:?}", val);
    });

    //let recieved = rx.recv().unwrap();
    //println!("Recieved: {:?}", recieved); 

    t.join();
    let mut recieved_status = false; 
    while recieved_status != true {
        match rx.try_recv() {
            Ok(recieved_value) => {
                println!("Recieved value is {:?}", recieved_value); 
                recieved_status = true; 
            },
            Err(_) => println!("I am doing some other stuff"),
        }
    }

   }