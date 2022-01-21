//Structs- custom data types
struct Person{
    first_name: String,
    last_name: String,
}

impl Person{
    //construct person
    fn new(first: &str, last:&str) -> Person{
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    //get full name
    fn full_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }
    //set last name
    fn set_last_name(&mut self, last:&str){
        self.last_name = last.to_string();
    }
    //name to tuple
    fn to_tuple(self) ->(String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    //instantiate person with p
    let mut p = Person::new("Kondwani","Gideon");
    println!("Person {}", p.full_name());
    p.set_last_name("Ngulube");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}