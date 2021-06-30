pub fn run() {
    // Print to console
    println!("===============PRINT.RS===============");
    println!("Hello from Print.rs file");

    // Example of basic string formatting
    println!("Number: {}, Number: {}", 1, 2);
    println!("My first name is {}, my last name is {}", "Casper", "Norspang");

    // Positional Arguments
    println!("{0} studies at {1}, {0} also works at {1}", "Casper", "AAU");

    // Named Arguments
    println!("{name} enjoys programming in {language}", name = "Casper", language = "Rust");

    // Placeholder traits
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal {:o}", 10, 10 , 10);

    // Placeholder Debug trait
    println!("{:?}", (12,true,"Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}