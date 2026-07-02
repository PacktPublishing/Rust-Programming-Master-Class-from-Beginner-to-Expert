/* Starting Code
#![feature(trait_alias)]

trait Printable {
    fn print(&self);
}

trait Calculable {
    fn calculate(&self) -> i32;


trait PrintAndCalculate = Printable + Calculable;

struct Data {
    value: i32,
}

impl Printable for Data {
    fn print(&self) {
        println!("Value: {}", self.value);
    }
}

impl Calculable for Data {
    fn calculate(&self) -> i32 {
        self.value * 2
    }
}

fn process<T: Printable + Calculable>(item: &T) {
    item.print();
    let result = item.calculate();
    println!("Calculated: {}", result);
}


fn main() {
    let data = Data { value: 42 };
    process(&data);
}
*/

#![feature(trait_alias)]

trait Printable {
    fn print(&self);
}

trait Calculable {
    fn calculate(&self) -> i32;
}

struct Data {
    value: i32,
}

impl Printable for Data {
    fn print(&self) {
        println!("Value: {}", self.value);
    }
}

impl Calculable for Data {
    fn calculate(&self) -> i32 {
        self.value * 2
    }
}

// Approach 1:
// trait PrintableAndCalculable: Printable + Calculable {}
// impl PrintableAndCalculable for Data {}

trait PrintableAndCalculable = Printable + Calculable;
// impl PrintableAndCalculable for Data {}  // gives an error of :expected trait, found trait alias `PrintableAndCalculable` not a trait

// fn process<T: Printable + Calculable>(item: &T) {
// fn process<T: PrintableAndCalculable>(item: &T) {    // Approach 1: 1. requires implementation for all types
fn process<T: PrintableAndCalculable>(item: &T) {
    // Approach 2: Requires the use of unstable feature
    item.print();
    let result = item.calculate();
    println!("Calculated: {}", result);
}

/* Key differences
                Super Trait                   |         Trait Alias
- It is a trait                               | - It is not a trait
- Any type which implements the super trait,  | - It is a way to just mention multiple traits
  must also implement the required traits     |

*/
fn main() {
    let data = Data { value: 42 };
    process(&data);
}
