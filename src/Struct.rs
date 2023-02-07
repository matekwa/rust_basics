//Tradiional struct
// struct color{
//     red: u8,
//     green: u8,
//     blue: u8
// }

//Tuple struct
// struct Color (u8,u8,u8); 

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    //Construct
    fn new(first: &str, last: &str) -> Person{
        Person {
        first_name: first.to_string(),
        last_name: last.to_string(),
        }
    }

    fn get_fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String){
        (self.first_name,self.last_name)
    }
}
pub fn run() {
    // let mut c = color{
    //     red : 255,
    //     green : 0,
    //     blue : 0
    // };
    // let c = Color(255,0,0);
    let mut p = Person::new("Tiffy", "Doe");
    p.set_last_name("Matekwa");

    println!("Person Tuple: {:?}", p.to_tuple());
}
