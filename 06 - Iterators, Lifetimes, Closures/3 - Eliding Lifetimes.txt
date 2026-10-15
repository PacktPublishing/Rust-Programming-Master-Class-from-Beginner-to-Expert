// -------------------------------------------
// 	        Lifetime Elision
// -------------------------------------------

/*
1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to
    all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
   the lifetime of self is assigned to all output lifetime parameters.
*/

//Example 1:
fn main() {
    let str_1 = "some str";

    let recieved_str = return_str(&str_1);
}
// Code with Lifetime Elision
// fn return_str(s_1: &str) -> &str {
//     s_1
// }

// Code without Lifetime Elision
fn return_str<'a>(s_1: &'a str) -> &'a str {
    s_1
}

// Example 2:
/* fn main() {
    let str_1 = "some str";
    let str_2 = "other str";
    let recieved_str = return_str(&str_1, &str_2);
}

fn return_str<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
    s_1
}
*/
