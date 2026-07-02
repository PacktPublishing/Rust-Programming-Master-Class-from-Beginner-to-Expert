   // -------------------------------------------
   // 			Web Programming Basics
   // ------------------------------------------- 


use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead}; 
use std::io::prelude;  
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap(); 
    //let stream = listener.accept(); 

    //println!("The stream {:?} \n The socket {:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0);  
    // for i in 0..10 {
    //     match listener.accept() {
    //         Ok((socket, addr)) => println!("The client info: {:?}", addr),
    //         Err(e) => println!("Couldn't get cliet: {:?}", e),
    //     }
    // }

    for stream in listener.incoming() {
        let stream = stream.unwrap();  
        handle_connection(stream);
    }
} 

fn handle_connection(mut stream: TcpStream) { 
    let buf_reader = BufReader::new(&mut stream); 

    let http_request = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|lines| !lines.is_empty())
    .collect::<Vec<String>>(); 

println!("Request: {:#?}", http_request);


    }