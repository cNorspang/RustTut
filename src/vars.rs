//Variables hold primetive data, or references to data
//Variables are immutable by default
//Rust is block-scoped

pub fn run() {
    let name = "Casper"; //Immutable, akin to const in JS
    let mut age = 21; //Mutable
    

    //name = "NotCasper"; not allowed, since name is immutable
    println!("===============VARS.RS===============");
    println!("My name is {}, and i'm {} years old", name, age);
    println!("YOU AGED UP!!");
    age = 22; //Allowed, since we explicitly stated age to be mutable
    println!("My name is {}, and i'm {} years old", name, age);


    //Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign mutliple vars
    let (my_name, my_age) = ("Casper", "21");
    println!("{} is {}", my_name, my_age);
}