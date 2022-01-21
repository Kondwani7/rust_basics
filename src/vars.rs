pub fn run(){
    //variables
    let profession = "software engineer";
    // make a variable mutable
    let mut experience = 1;
    let age = 21;
    println!("Hi. I am {0} years old have worked as a {1} for over {2} years", age ,profession, experience);
    experience = 2;
    println!("Hi. I am {0} years old have worked as a {1} for over {2} years", age ,profession, experience);
    //constant
    const ID: i32 = 002;
    println!("User id: {}", ID);
    //multiple varabiles assignment
    let (name, salary) = ("Kondwani", 2600);
    println!("{0} works in the Data team and earns {1} a month", name, salary);
}