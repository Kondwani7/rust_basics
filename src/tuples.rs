pub fn run(){
    let person: (&str, &str ,i8) = ("Joshua", "Lusaka", 22);
    println!("{0} is from {1} and is {2} years old", person.0, person.1, person.2);
}