pub fn run(){
    //default i32
    let _x = 1;
    //default f64
    let _y = 2.5;
    //add a explicit data type
    let _z: i64 = 4304595904;
    //println!("Z:{}", z);
    //get the max value of a data type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    //boolean
    let payment = 6.00;
    let is_paid_subscriber = payment > 5.00;
    let is_premium_subscriber = payment > 10.00;
    //string/character
    let _a = "hi";
    //emoji, which is a unicode
    let smiley_face = '\u{1F600}';
    let grin = '\u{1F604}';
    let laugh = '\u{1F923}';
    println!("{:?}", (is_paid_subscriber,is_premium_subscriber,smiley_face, grin, laugh));
}