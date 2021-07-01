//Reference pointers - poiting to a resource in memory

pub fn run(){
    println!("===============POINTER_REF.RS===============");
    
    // Primitive Array - Copies value, does not "delete" original variable
    let arr1 = [1,2,3];
    let arr2 = arr1; 
    println!("Array Values: {:?}",(arr1, arr2));

    // Non-primitive datatypes - If you assign another variable to a piece of data, the first
    // variable will no longer hold the value. Use references instead (&), to point to the original data

    let vec1 = vec![1,2,3];
    //let vec2 = vec1; This basically deletes "vec1"

    //Now we make vec2 hold a reference to vec1 instead, not "deleting" vec1
    let vec2 = &vec1;

    //Note that when we want to use "vec1", we need to pass a reference to vec1 instead
    println!("Vector Values: {:?}",(&vec1, vec2));

}
