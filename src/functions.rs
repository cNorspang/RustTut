pub fn run() {
    println!("===============FUNCTIONS.RS===============");
    greeting("Guten Tag", "Norspang");

    // Bind function values to vars, like functional languages
    let res = add_two_numbers(1, 2);
    println!("Sum: {}", res);

    //Closure - Outside vars can be used, without passing, as this is in the same scope
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure
     Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, it's nice to meet you", greet, name);
}

// No semicolon = return value???
fn add_two_numbers(n1:i32, n2:i32) -> i32 {
    //Could be written with explicit "return" keyword, but this is not idomatic Rust
    n1 + n2
}