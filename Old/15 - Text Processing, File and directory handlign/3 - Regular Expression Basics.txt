   // -------------------------------------------
   // 			Regular Expression Basics 
   // ------------------------------------------- 
 
   extern crate regex; 
   use regex::Regex; 
   fn main() {
    // let re = Regex::new(r"[prt].ain").unwrap(); 
    // let text = "rrrain spain none";      

    // // println!("The text has a match {:?}", re.is_match(text)); 
    // // println!("The text has a match {:?}", re.find(text)); 

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"gr[ae]y").unwrap(); 
    // let text = "gray grey graye"; 

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"[^a-z]ain").unwrap(); 
    // let text = "main pain tain rain but not 0ain"; 

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }


    // let re = Regex::new(r"\d\d\d\d\d\d").unwrap(); 
    // let re = Regex::new(r"^\d......").unwrap(); 
    // let text = "My phone number is 816030 and the second phone number is 816694"; 

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"^aba").unwrap(); 
    // let text  = "ba abaa bc"; 

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    //let re = Regex::new(r"bc$").unwrap(); 
    // let re = Regex::new(r"^bc$").unwrap(); 
    // let text = "bc";
    
    // //let text = "aba abc bc"; 

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // } 

    // let re = Regex::new(r"^\d\d$"); 
    // let text = "89";

    let re = Regex::new(r"\b\w*").unwrap();
    let text = "Hi my name is nouman"; 

    for cap in re.captures_iter(text) {
        println!("match: {:?}", &cap[0]);
    }
}