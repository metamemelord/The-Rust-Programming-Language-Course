pub fn init() {
    println!("\nStatic Dispatch:");
    let a = 123;
    let b = "hello".to_string();

    pprint(a);
    pprint(b);
}

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String {}", *self)
    }
}

// THIS function is monomorphized
fn pprint<T: Printable>(z: T) {
    println!("{}", z.format());
}
