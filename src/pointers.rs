//point to a resource in memory
pub fn run(){
    let arr1 = [12, 16, 22];
    let arr_ref = arr1;
    //vector pointer
    let vec1 = vec![10, 12, 16, 22];
    let vec_ref = &vec1;
    println!("Array & its pointer: {:?}", (&arr1, arr_ref));
    println!("Vector & its pointer: {:?}", (&vec1, vec_ref));
}
