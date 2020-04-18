pub fn init() {
    let ds = do_something;
    ds();

    let inc = |x: i32| -> i32 { x + 1 };
    let mut x = 10;
    println!("x={}", x);
    x = inc(x);
    println!("x now {}", x);

    let mut a_big_number = 100;
    // This closure gets a shared (non mutable) reference to a_big_number
    let add_a_big_number = |x: i32| -> i32 { x + a_big_number };

    println!("{}", add_a_big_number(10));
    // This errs out, because we have a non-mutable borrow earlier
    // let borrow = &mut a_big_number;

    let mut another_big_number = 200;
    {
        // A new scope
        // This closure gets a shared (non mutable) reference to a_big_number
        let add_another_big_number = |x: i32| -> i32 { x + another_big_number };

        println!("{}", add_another_big_number(10));
    }
    // This does err out, because a non-mutable ref to another_big_number ends at previous line
    let borrow = &mut another_big_number;

    // T: by value
    // &T: by reference
    // &mut T: by mutable reference

    let sqr = |x: &mut i32| *x = *x * *x;
    let mut val = 9;
    let org = val;
    sqr(&mut val);
    println!("{} is the square of {}", val, org);
}

fn do_something() {
    println!("Doing...")
}
