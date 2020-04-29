pub fn init() {
    println!("\nDynamic dispatch:");
    let a = 123;
    let b = "hello".to_string();

    pprint(&a);
    pprint(&b);

    let shapes: [&dyn Shape; 4] = [&Circle(1.0), &Square(4.5), &Circle(2.3), &Square(4f64)];

    for shape in shapes.iter() {
        println!("Area of this shape is {}", shape.area());
    }
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

fn pprint(z: &dyn Printable) {
    println!("{}", z.format());
}

struct Circle(f64);
struct Square(f64);

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}
