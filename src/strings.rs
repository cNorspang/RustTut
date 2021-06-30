//Primitive str = Immutable, fixed-lenght stored on stack
// String = Growable, heap-allocated data structure - Use when the string needs to be modified

pub fn run(){
    let mut nonprimitive_hello = String::from("hello");
    let primitive_hello = "Hello";

    println!("===============STRINGS.RS===============");
    //Get lenght of string
    println!("Length of String: {}",nonprimitive_hello.len());

    //Add one char to string
    nonprimitive_hello.push('🤔');

    //Add a string
    nonprimitive_hello.push_str("Smileys are cool");

    //Get size of string
    println!("Capacity: {}", nonprimitive_hello.capacity());

    //See if a string is emtpy
    println!("Is Empty: {}", nonprimitive_hello.is_empty());

    //See if string contain
    println!("Contains 🤔: {}", nonprimitive_hello.contains("🤔"));

    println!("{}", nonprimitive_hello);
    println!("{}", primitive_hello);
}