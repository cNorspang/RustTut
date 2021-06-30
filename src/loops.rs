pub fn run() {
    let mut count = 0;
    let mut fb_string = String::from("");

    println!("===============LOOPS.RS===============");

    // Infinite Loop, like "while true"
    loop {
        count += 1;
        println!("From Infinite - Number: {}", count);

        if count == 20 {
            count += 1;
            break;
        }
    }

    //Normal while loop
    while count <= 100 {
        fb_string.clear();
        if count % 3 == 0 {fb_string.push_str("Fizz")};
        if count % 5 == 0 {fb_string.push_str("Buzz")};

        if !fb_string.is_empty() {println!("{}", fb_string); count += 1;}
        else {println!("{}", count); count += 1};
    }

    //For Range (Range 1 to 100)
    for x in 1..100 {
        fb_string.clear();
        if x % 3 == 0 {fb_string.push_str("Fizz")};
        if x % 5 == 0 {fb_string.push_str("Buzz")};

        if !fb_string.is_empty() {println!("{}", fb_string); count += 1;}
        else {println!("{}", x);};
    }
}