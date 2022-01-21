pub fn run(){
    let mut salaries: [i32; 6] = [25, 10, 2, 3, 10, 20];
    salaries[2] = 4;
    println!("salaries: {:?}", salaries);
    println!("first salary: {}", salaries[0]);
    //array length
    println!("Array length: {}", salaries.len());
    //get memory allocated to an array: 4 bits per unit
    println!("Memory allocated to salaries array: {} ", std::mem::size_of_val(&salaries));
    //get slices
    let slice: &[i32] = &salaries[0..2];
    println!("Slice first 2 elements: {:?}",slice);
}