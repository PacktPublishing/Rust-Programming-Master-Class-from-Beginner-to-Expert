//    // -------------------------------------------
//    // 			Async Await (Tasks, Select)
//    //           tokio = {version = "1.17", features = ["full"]}
//    // -------------------------------------------

// #[tokio::main]
// async fn main() { 
//     let mut handles = vec![]; 
//     println!("This code is not part of the async block"); 
//     let s1 = String::from("Huge Computation function"); 
//     let s2 = String::from("Simpler Computation function");  
//     let aw1 = tokio::spawn(async move {
//         huge_computation(s1).await;
//     });
//     handles.push(aw1);

//     let aw2 = tokio::spawn(async move {
//         simpler_computation(s2).await;
//     }); 
//     handles.push(aw2); 

//     for handle in handles {
//         handle.await.unwrap(); 
//     }
//     println!("All tasks are now completed"); 
// }

// async fn huge_computation(s: String) {
//     println!("{:?} has started", s); 
//     for i in 0..100_000_000{

//     }
//     println!("{:?} is now completed", s);
// }
// async fn simpler_computation(s: String) {
//     println!("{:?} has started", s); 
//     println!("{:?} is now completed", s); 
// }

use tokio::select; 
#[tokio::main] 
async fn main() {
    // tokio::select! {
    //     _ = function_1() => println!("Function 1 is completed first"), 
    //     _ = function_2() => println!("Function 2 is completed first"), 
    // };

    // let aw1 = tokio::spawn(async move {
    //     function_1().await;
    // }); 

    // let aw2 = tokio::spawn(async move {
    //     function_2().await;
    // }); 

    // tokio::select! {
    //     _ = aw1 => println!("Function 1 is selected"), 
    //     _ = aw2 => println!("Function 2 is selected"),
    // }; 

    tokio::join!{
        function_1(), 
        function_2(),
    };

    
}

async fn function_1() {
    println!("Function 1 has started"); 
    for i in 0..100_000_000 {

    }
    println!("Function 1 has ended"); 
} 

async fn function_2() {
    println!("Function 2 has started"); 
    println!("Function 2 has ended");
}