   // -------------------------------------------
   // 			Initializing Struct Instance 
   // -------------------------------------------
use project::Student;
fn main() {
    /*
    let new_student = Student {
        id: 11,
        name: "joseph".to_string(),
        age: 20,
    };
    */




    let new_student = Student::new("joseph".to_string())
    .unwrap_or_default(); // next example, make sure to remove semicolon from the statement above

    println!("{:?}", new_student);


    
    let new_student = Student::default();
    println!("{:?}", new_student);
     
    let new_student = Student {
    age: 12,
    .. Default::default()
    };
}



// code for library
#[derive(Debug, Default)] 
pub struct Student {
    pub id: u8,

    pub name: String,
    pub age: u8,
}

impl Student {
/*
    pub fn new(std_name: String) -> Self {
        Self {
            id: 0,
            name: std_name,
            age: 20,
        }
    }
*/     


    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Ok(Self {
                id: 0,
                name: std_name,
                age: 20,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

/* 
impl Default for Student {
    fn default() -> Self {
        Self {
            id: 0,
            name: "unknown".to_string(),
            age: 20,
        }
    }
}

*/

