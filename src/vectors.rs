// Resizeable arrays

//Brining in libaries
use std::mem;

pub fn run(){
    //Make an array with a lenght of 5, that stores 32 bit signed ints
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("===============VECTORS.RS===============");
    println!("{:?}", numbers);
    println!("This is the num at index 3 of Vector: {}", numbers[3]);

    //Arrays can be mutable - i.e You can reassign values, but not add to arrays
    numbers[0] = 32;
    println!("Vector after change {:?}", numbers);

    //Get lenght of array
    println!("Vector Lenght: {}", numbers.len());

    //Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice of array (Note, reference type)
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice);

    //Pushing to a Vector
    numbers.push(6);
    println!("Vector after adding 6{:?}", numbers);

    //Popping last value
    numbers.pop();
    println!("Vector after popping {:?}", numbers);

    //Looping trough vectors
    for x in numbers.iter() {
        println!("Number {}", x);
    }

    //Looping and mutating Vector, adding 1 to each value
    for x in numbers.iter_mut() {
        *x += 1;
    }

    println!("Vector after adding 1 to each entrty: {:?}", numbers);


}