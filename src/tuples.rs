// Tuples group togetether values of diffrent types
// Max 12 elements

pub fn run(){
    let person: (&str, &str, i8) = ("Casper", "Aalborg", 21);

    println!("===============TULPES.RS===============");
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}