pub fn run(){
    let mut _count =0;
    //while loop
    /*
    while _count <=100 {
        if count %15 ==0{
            println!("multiples of 15");
        } else if count % 3 ==0{
            println!("multiple of 3");
        } else{
            println!("Count: {}", count);
        }
    }*/
    //for loop
    for x in 0..100{
        if x % 15 == 0 {
            println!("Mutliple of 15");
        }else if x % 3 ==0 {
            println!("Mutliple of 3");
        }else{
            println!("{}",x);
        }
    }
    println!("loops");
}