pub fn run(){
    println!("hello from the print.rs file");
    //basic formating
    println!("{0} is {1} years old. Moreover, {0} loves to read {book}",
             "Kondwani",
             21, 
             book="The intelligent investor");
    //place holder traits
    println!("Binary:{:b} Hex:{:x}, Octal:{:o}", 10, 10, 10);
    //place holder for debug traits
    println!("{:?}", (12, true, "hello"));
    //basic math
    println!("21 * 10: {}", 21 * 5);
}