// Like C arrays, fixed list, one datatype

pub fn run(){
    //Make an array with a lenght of 5, that stores 32 bit signed ints
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("===============ARRAYS.RS===============");
    println!("{:?}", numbers);
    println!("This is the num at index 3 of array: {}", numbers[3]);

    //Arrays can be mutable - i.e You can reassign values, but not add to arrays
    numbers[0] = 32;
    println!("Array after change {:?}", numbers);

    //Get lenght of array
    println!("Array Lenght: {}", numbers.len());
}