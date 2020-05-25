#![allow(unused_variables)]

pub fn init() {
    println!("\nBorrowing:");

    let printer = |x: &Vec<i32>| {
        for i in x {
            print!("{} ", i);
        }
        println!("");
    };

    let v = vec![1, 5, 8, 4, 3];
    printer(&v);

    let mut a = 40;
    let b = &mut a;
    *b += 2;
    print_type_of(&b);
    println!("a = {}", a);

    let mut x = vec![23, 6];

    for i in &mut x {
        // x.push(10); Cannot have 2 mutable borrows.
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
