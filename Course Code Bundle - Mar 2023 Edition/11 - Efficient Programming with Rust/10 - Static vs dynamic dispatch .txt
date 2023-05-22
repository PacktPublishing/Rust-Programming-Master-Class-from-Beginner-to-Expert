// -------------------------------------------
//           	- Static vs Dynamic Dispatch
// ------------------------------------------- 
/* 
trait Print{
    fn print(&self);
}
impl Print for String{
    fn print(&self) {
        println!("I got the value of \"{}\"", self);
    }

}

impl Print for i32 {
    fn print(&self) {
        println!("I got the value of {}", self);
    }
}

/* 
fn display_i32 (x: i32){
    x.print();
}
fn display_String(x:String) {
    x.print();
}

*/
fn display<T: Print>(x: T) {
    x.print();
}

fn main(){
    display(5); 
    display("Hello rust".to_string());
}
*/ 


/* 
#[derive(Debug)]
struct Person {
    name: String,
}
struct Student{
    name: String, 
}

trait info{
    fn info(&self);
}

impl info for Person {
    fn info(&self) {
        println!("{:?}", self.name)
}

}

impl info for Student {
    fn info(&self) {
        println!("{:?}", self.name);
    }    
}

fn static_dispatch<T: info>(t: &T) {
    t.info();
}

fn dynamic_dispatch(t: &dyn info) {
    t.info();
}

fn main(){
    let p1 = Person{
        name: "Nouman".to_string(),
    }; 

    let p2 = Student {
        name: "Affan".to_string(),        
    }; 
    static_dispatch(&p1); 
    static_dispatch(&p2);

    dynamic_dispatch(&p1); 
    dynamic_dispatch(&p2);
}
*/ 


trait Print {
    fn print(&self);
}


impl Print for String {
    fn print(&self) {
        println!("I got the value \"{}\"", self);
    }
}

impl Print for i32 {
    fn print(&self) {
        println!("I got the value {}", self);
    }    
}

fn static_display<T: Print>(x: T) {
    x.print();
}

fn display_dynamic(x: Vec<Box<dyn Print>>) {
    for i in x {
        i.print();
    }
}
/* 
fn display_i32(x: i32) {
    x.print()
}

fn display_string(x: string) {
    x.print()
}

*/

fn main() {
    static_display(5);  
    static_display("Hello rust".to_string()); 


    display_dynamic(vec![Box::new("Hello world".to_string()), Box::new(10)]);

}
