pub fn run(){
    let mut greet = String::from("hi developer ");
    //get length
    println!("length:{}", greet.len());
    //push one character
    //greet.push("k");
    //push characters
    greet.push_str("kondwani");
    println!("{}",greet);
    println!("{}", greet.capacity());
    //check if string is empty
    println!("Is empty: {}", greet.is_empty());
    //check if it contains certain values
    println!("Contains 'wani' {}", greet.contains("wani"));
    //replace string
    println!("Replace:{}", greet.replace("developer", "data engineer"));
    //loop through  ever word
    let story=  "Jerry went to the mall to buy some yeezies. He could not find his size so he opted for the air jordans instead";
    for word in story.split_whitespace(){
        println!("{}",word);
    }
    //create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    //assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);


}