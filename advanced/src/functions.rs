pub fn init() {
    println!("\nFunctions:");
    pprint(20);

    let mut x = 20;
    inc(&mut x);
    print!("x = {}, x + 10 = ", x);
    pprint(add(x, 10));

    // Methods
}

fn pprint(x: i32) {
    println!("{}", x);
}

fn inc(x: &mut i32) {
    *x += 1;
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
