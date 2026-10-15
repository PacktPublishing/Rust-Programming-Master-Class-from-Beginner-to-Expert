
   // -------------------------------------------
   // 			Regexes- Repeatitions and Quantifiers
   // -------------------------------------------  
   
   extern crate regex; 
   use regex::Regex;

   fn main(){
    //let re = Regex::new(r"a?aa").unwrap(); 
    //let text = "aa aaa"; 

    // let re = Regex::new(r"ba?").unwrap(); 
    // let text = "a ba b ba"; 

    // let re = Regex::new(r"\w?\w?\w?.rs").unwrap(); 
    // let text = "fil.rs t1.rs file.rs";

    // for cap in re.captures_iter(text) {
    //     println!("Match: {} ", &cap[0]);
    // } 

    // let re = Regex::new(r"a+").unwrap(); 
    // let text = "a aa aaa baab bab"; 

    // let re = Regex::new(r"\w+\.gif").unwrap(); 
    // let text = "image1.gif and background.gif";
    
    // let re = Regex::new(r"ab*").unwrap(); 
    // let text = "a ab abbbbb";

    // for cap in re.captures_iter(text) {
    //     println!("Match: {} ", &cap[0]);
    // }

//    let re = Regex::new(r"\b\w{3,5}\b").unwrap();  
//    let text = "Hello i think you are happy becuase i have a gift for you"; 
    
    // let re = Regex::new(r"\b\d{1,3}\.\d{1,3}\b").unwrap(); 
    // let text = "921.583 0.0 1456.25"; 

//     let re = Regex::new(r"\d{3,}").unwrap(); 
//     let text = "5321 500 5698 12"; 

//    for cap in re.captures_iter(text) {
//     println!("Match: {} ", &cap[0]);
// }


    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap(); 
    let text = "2012-03-14, 2013-01-01 and 2014-07-05"; 

    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}, the whole: {}", &cap[2], &cap[3], &cap[1], &cap[0]);
    }

   }