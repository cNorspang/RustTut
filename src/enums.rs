// Enums are Enums

enum Movement {
    Up, 
    Down, 
    Left, 
    Right
}

fn move_avatar(m: Movement) {
    // Perform something depending on input 
    // Match like switch
    match m {
        Movement::Up => println!("You moved up"),
        Movement::Down => println!("You moved down"),
        Movement::Left => println!("You moved left"),
        Movement::Right => println!("You moved right")
    }
}

pub fn run(){
    println!("===============ENUMS.RS===============");
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}