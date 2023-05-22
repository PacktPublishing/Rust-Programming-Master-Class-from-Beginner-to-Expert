   // -------------------------------------------
   // 			 Thread Park
   // ------------------------------------------- 

   use std::thread; 
   use std::time::Duration;
   fn main() {
    let job_1 = thread::spawn(|| {
        println!("-- Job 1 has started -- "); 
        println!("Waiting for job 2 to complete"); 
        //thread::park_timeout(Duration::from_secs(2));
        //thread::sleep(Duration::from_secs(2));   
        thread::yield_now();

        println!("-- Job 1 resumed --"); 
        println!("-- Job 1 finished"); 
    });

    let job_2 = thread::spawn(|| {
        println!("-- Job 2 started --"); 
        println!(" -- Job 2 finished --"); 
    }); 
    job_2.join().unwrap(); 
    println!("Job 2 is now completed"); 
    println!("Job 1 will now resume"); 
    job_1.thread().unpark(); 
    job_1.join().unwrap();
   }
