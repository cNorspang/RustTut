pub fn run(){
    println!("===============CLI.RS===============");
    // Note, First arg always path of exe
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();

    if command.contains(&"-d".to_string()) {
        println!("You're in debug mode now")
    }
}