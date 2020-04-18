pub fn intro() {
    println!("\nStrings:");
    let s: &'static str = "hello there!"; // &str = string slice
                                          // cannot reassign s or index it
                                          // s = "lol" or s[0] is invalid
    for c in s.chars() {
        print!("{}\t", c);
    }
    println!();

    for c in s.chars().rev() {
        print!("{}\t", c);
    }
    println!();

    if let Some(first_char) = s.chars().nth(0) {
        println!("First letter: {}", first_char);
    }

    // HEAP Allocated -> String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        a += 1;
    }
    letters.push_str("!!!");
    println!("{}", letters);
}

pub fn format_macro() {
    let time = "evening";
    let greeting = format!("Hello! Good {}!", time);
    println!("{}", greeting);

    let whats_up = format!("{0}!, {1} {0} ", "yo", "what's up");
    println!("{}?", whats_up);

    let jb = format!(
        "The name's {lastname}, {firstname} {lastname}.",
        firstname = "James",
        lastname = "Bond"
    );
    println!("{}", jb);

    // Errs out, because 2nd arg is unused
    //  let numbers = format!("{0} {} {2} {wew}", "vakti", "rendu", "mudu", wew = "lol");

    let numbers = format!("{0} {} {2} {} {wew}", "vakti", "rendu", "mudu", wew = "lol");
    println!("{}", numbers);
}
