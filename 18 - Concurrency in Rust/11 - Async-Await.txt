   // -------------------------------------------
   // 			Async Await basics
   //           tokio = {version = "1.17", features = ["full"]} 			
   // -------------------------------------------  
   
   async fn printing(){
    println!("I am async function");
   }

   #[tokio::main]
   async fn main(){ 
    let x = printing();
    println!("The has not being polled yet"); 
    drop(x);
    //x.await;

   }