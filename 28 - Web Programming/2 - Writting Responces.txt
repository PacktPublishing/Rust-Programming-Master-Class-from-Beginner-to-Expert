   // -------------------------------------------
   // 			Writing Responces 
   // -------------------------------------------

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead,Write}; 
use std::io::prelude;  
use std::fs; 

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap(); 
    for stream in listener.incoming() {
        let stream = stream.unwrap();  
        handle_connection(stream);
    }
} 

fn handle_connection(mut stream: TcpStream) { 
    let buf_reader = BufReader::new(&mut stream); 

//     let http_request = buf_reader
//     .lines()
//     .map(|result| result.unwrap())
//     .take_while(|lines| !lines.is_empty())
//     .collect::<Vec<String>>(); 

// println!("Request: {:#?}", http_request); 

/* 
Responce Syntax 

HTTP-Version Status-Code Reason-Phrase CRLF 
headers CRLF
message-body 

ex: HTTP/1.1 200 OK\r\n\r\n
*/



// let responce = "HTTP/1.1 200 OK\r\n\r\n"; 
// stream.write(responce.as_bytes()).unwrap(); 
// stream.flush().unwrap(); 


// let status_line = "HTTP/1.1 200 OK \r\n"; 
// let contents = fs::read_to_string("index.html").unwrap();  
// let length = contents.len(); 

// let responce = 
// format!("{} Contents-Length: {}\r\n\r\n{}", status_line, length, contents); 
// stream.write_all(responce.as_bytes()).unwrap(); 
// stream.flush().unwrap();

let mut request_line = buf_reader.lines().next(); 
let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
    "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index.html")), 
    "GET /page1 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("page1.html")), 
    "GET /page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("page2.html")), 
    _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")), 
};

let contents = fs::read_to_string(file_name.unwrap()).unwrap(); 
let responce = format!("{} Content-Length: {}\r\n\r\n{}", status_line.unwrap(), contents.len(), contents); 

stream.write_all(responce.as_bytes()).unwrap(); 
stream.flush().unwrap();    
}