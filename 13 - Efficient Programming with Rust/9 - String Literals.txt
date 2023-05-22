// -------------------------------------------
//           	- String Literal
// ------------------------------------------- 
fn main() {
    let str = r"The main said _Hello world_ \n \t ' "; 
    println!("{}", str);

    let jason_str = "{
        \" name \": \"Micheal\", 
        \"age\": 40, 
        \"sex\": Male
    }";

    let jason_str1 = r#"{
        " name ": "Micheal", 
        "age": 40, 
        "sex": Male
    }"#;

    let str = r###"Hello"## World!"###;

    
    // Exercise for you

    let string1 = r#"""#; // " 
    let string2 = r#""""""""#; // """"""
    let string3 = r#" He asked,"Is rust awesome?"""#; // He asked,"Is rust awesome?" 
    println!("{}", string1);
    println!("{}", string2);
    println!("{}", string3);
}