use std::env;
pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Kondwani";
    let status = "100%";
    
    if command == "hello"{
        println!("HI {}, how are you", name);
    }else if command == "status" {
        println!("Status is {}", status);
    }else{
        println!("Invalid command");
    }
    //to run command: cargo run "command"

}