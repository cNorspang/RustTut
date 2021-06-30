

pub fn run() {
    let age:u8 = 18;
    let check_id:bool = false;
    let knows_person_of_age = true;

    println!("===============CONDITIONALS.RS===============");

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Of legal drinking age")
    } 
    else if age < 21 && check_id {
        println!("You're under 21, not of legal driking age")
    } else {
        println!("Id please");
    }

    // Shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}