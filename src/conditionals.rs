pub fn run(){
    let age: u8 = 19;
    let check_id: bool = true;
    let check_provision_id: bool = true;
    let adult_supervisor = true;
    //if/else
    if age>= 21 && check_id {
        println!("You allowed to drive");
    }else if age >=18 && check_provision_id{
        println!("You can only drive with an adult supervisor");
    }else if age < 21 &&check_id{
        println!("You are too young to drive");
    }else{
        println!("show me your id");
    }
}