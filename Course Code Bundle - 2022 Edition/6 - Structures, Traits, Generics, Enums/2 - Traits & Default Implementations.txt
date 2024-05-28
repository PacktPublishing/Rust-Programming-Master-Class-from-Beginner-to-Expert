   // -------------------------------------------
   // 			Traits
   //           	- General explaination
   // 			- Default function implementation
   // -------------------------------------------

/*

struct Person{
    citizenship: String, 
    name: String,
    age: u8, 
    gender: char, 
    salary: i32,
}

struct Student{
    name_std: String,
    age: u8, 
    sex: char,
    country: String,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);     

    // we can have multiple functions inside a trait
    fn country_info(&self) -> &str; 
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&(self.name), self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &(self.name) 
    }
    
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&(self.name_std), self.age, self.sex)
    }

    fn country_info(&self) -> &str {  
        &(self.name_std) 
    }
    
}
fn main () 
{
    let person1 = Person {
        name: String::from("Nouman Azam"),       
        citizenship: String::from("Pakistan"), 
        age: 40, 
        gender: 'M',
        salary: 40_000,
    };   
    
    let student1 = Student {
        name_std: String::from("Affan Azam"),       
        age: 15, 
        sex: 'M',
        country: String::from("USA"),
    };   
    println!("The basic info include {:?}", person1.info()); 
    println!("The basic info for the student is {:?}", student1.info()); 
}

*/

// Example 2: more on traits and learning about default implementation 
struct Circle {
    raduis: f32, 
}

struct Rectangle {
    length: f32, 
    width: f32,
}

trait GeneralInfo {    
    fn area(&self); 
    
    /*fn area(&self) {
        println!("I am not implemented for the type"); 
    }
    */
    fn perimeter(&self);
}


impl GeneralInfo for Circle {    
    fn area(&self) {
        let area_of_circle = 3.14 * (self.raduis * self.raduis); 
        println!("The area of the circle is {}", area_of_circle); 
    } 

    fn perimeter(&self)  {
        let circumference = 2.0 * 3.14 * self.raduis; 
        println!("The circumference of the circle is  {}", circumference); 
    } 

}


impl GeneralInfo for Rectangle {   
    fn area(&self) {
        let area_of_rect = self.length * self.width; 
        println!("The area of the rectange is {}", area_of_rect);
    } 

    fn perimeter(&self) {
       let perimeter_of_rect = 2.0 * (self.length + self.width); 
       println!("The area of the rectange is {}", perimeter_of_rect); 
    } 

}


fn main(){
    let c1 = Circle {
        raduis: 3.2,
    };

    let r1 = Rectangle {
        width: 5.0, 
        length: 4.0,
    };

    c1.area(); 
    c1.perimeter();

    r1.area(); 
    r1.perimeter(); 
     

}
