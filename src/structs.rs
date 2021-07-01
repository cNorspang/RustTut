//Used to create custom data-types, like in C
//Uppercase Convention

//Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//Tuple struct
struct ColorTuple(u8, u8, u8);

//Struct with some associated function
struct Person {
    first_name: String,
    last_name: String
}

impl Person{
    //Constructor
    fn new(first:&str, last:&str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //Function to get name (&self is like this) 
    fn get_name(&self) -> String{
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name, not the passing of &mut self, to make it a mutable struct
    fn set_last_name(&mut self, new_last_name: &str){
        self.last_name = new_last_name.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    println!("===============STRUCTS.RS===============");

    // Traditional Struct
    let mut red_color = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Color before green: {} {} {}", red_color.red, red_color.green, red_color.blue);

    //Members of a mutable struct are mutable, via dotnotation
    red_color.green = 255;
    println!("Color after green: {} {} {}", red_color.red, red_color.green, red_color.blue);

    //Tuple struct
    let mut blue_color = ColorTuple(0,255,0);
    println!("Color before green: {} {} {}", blue_color.0, blue_color.1, blue_color.2);

    blue_color.2 = 255;
    println!("Color after green: {} {} {}", blue_color.0, blue_color.1, blue_color.2);

    //All related to person
    let mut me = Person::new("Casper", "Norspang");
    println!("First name: {}, Last name: {}", me.first_name, me.last_name);

    let my_full_name = me.get_name();
    println!("Full name {}", my_full_name);

    me.set_last_name("Stadig Norspang");
    let my_full_name = me.get_name();
    println!("Full name {}", my_full_name);

}