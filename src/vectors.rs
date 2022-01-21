pub fn run(){
    //use std::mem;
    let mut salaries: Vec<i32> = vec![25, 10, 2, 3, 10,89,20];
    salaries[2] = 4;
    //add to vector
    salaries.push(2);
    salaries.push(1000);
    //remove last element
    //salaries.pop();

    println!("salaries: {:?}", salaries);
    println!("first salary: {}", salaries[0]);
    //array length
    println!("vector length: {}", salaries.len());
    //get memory allocated to an array: 4 bits per unit
    println!("Memory allocated to salaries vector: {} ", std::mem::size_of_val(&salaries));
    //get slices
    let slice: &[i32] = &salaries[0..2];
    println!("Slice first 2 elements: {:?}",slice);
    /*loop through the vectors
    for x in salaries.iter(){
        println!("Salaries: {}", x);
    }*/
    //pay rise by 200%
    for y in salaries.iter_mut(){
        *y *= 2;
        println!("200% salary pay rise:{}", y);
    }
}