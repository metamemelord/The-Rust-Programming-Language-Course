#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

use std::mem;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn to_string(self) -> String {
        format!("Point {{x: {}, y: {}}}", self.x, self.y)
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn to_string(self) -> String {
        format!(
            "Line {{start: {}, end: {}}}",
            self.start.to_string(),
            self.end.to_string()
        )
    }
}

fn structs() {
    let p = Point { x: 3.0, y: 3.0 };
    let p2 = Point { x: 5.2, y: 1.0 };

    // Errors out as the value is moved here!!!!!!!!!!
    // println!("Line: {} -> {}", p.to_string(), p2.to_string());

    let line = Line { start: p, end: p2 };
    println!("Structs:\n{}", line.to_string());
}

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // Tuple
    CMYKColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // Struct
}

impl Color {
    fn to_string(self) -> String {
        match self {
            Color::Red => "RED".to_string(),
            Color::Green => "GREEN".to_string(),
            Color::Blue => "BLUE".to_string(),
            Color::RGBColor(0, 0, 0)
            | Color::CMYKColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "BLACK".to_string(),
            Color::RGBColor(r, g, b) => format!("rbg({},{},{})", r, g, b),
            Color::CMYKColor {
                cyan: c,
                magenta: m,
                yellow: y,
                black: k,
            } => format!("cmyk({},{},{},{})", c, m, y, k),
        }
    }
}

fn enums() {
    let c: Color = Color::CMYKColor {
        cyan: 2,
        magenta: 25,
        yellow: 41,
        black: 200,
    };
    println!("\nEnums: {}", c.to_string());
}

union IoF {
    i: i32,
    f: f32,
}

fn unions() {
    let iof = IoF { i: 10 };

    // Need to be put in an unsafe block.
    unsafe {
        println!("\nUnion: {}", iof.i);
    }
}

fn options() {
    let x = 3.0;
    let y = 1.51;

    println!("\nOptions:");

    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Failed to divide by zero"),
    };

    if let Some(z) = result {
        println!("From if statement: {}/{}={}", x, y, z);
    }

    while let Some(z) = result {
        println!("From while loop: {}/{}={}", x, y, z);
        break;
    }
}

fn array() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nArrays:\nlen(a) = {}", a.len());
    a[0] = 23;
    println!("a[0] = {}", a[0]);
    println!("{:?}", a);
    if a == [23, 2, 3, 4, 5] {
        println!("Equal arrays")
    }

    for x in &a {
        print!("{} ", x);
    }
    // Declare an array with same values.
    let mut b = [1f64; 10]; // [1; 10] takes data type as default, i.e. i32.
    b[3] = 2.0;
    println!("{:?}", b);
    println!("Size of b: {}", mem::size_of_val(&b));

    // Array of arrays: 2D array
    let c: [[f32; 4]; 3] = [[0.0; 4]; 3];
    println!("{:?}", c);
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("\nVectors:\nlen(a) = {}", a.len());

    // Addressing should be done using usize isize
    let idx: usize = 0;

    a[idx] = 20;
    println!("{:?}", a[idx]);

    // a.get(idx:usize) returns an Option
    match a.get(2) {
        Some(ele) => println!("a[2] = {}", ele),
        None => println!("a[2] doesn't exist! Out of bound, friend!"),
    };

    for x in &a {
        print!("{} ", x);
    }
    println!("");

    println!("{:?}", a);
    match a.pop() {
        Some(ele) => println!("Popped value: {}", ele),
        None => println!("Underflow fren!"),
    };
    println!("{:?}", a);

    println!("Using option with while to pop elements");
    while let Some(z) = a.pop() {
        println!("Popped value: {}", z);
    }
    println!("Popped everything");
}

fn pr(sl: &[i32]) {
    println!("{:?}", sl);
}

fn pr_mut(sl: &mut [i32]) {
    sl[0] = 234;
}

fn slices() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nSlices:");
    pr(&a[0..3]);
    pr_mut(&mut a[1..3]);
    println!("{:?}", a);
}

fn strings() {
    println!("\nStrings:");
    let s: &'static str = "hello!"; // &str => string slice
                                    // Cannot re-assign s = "lol";
                                    // Cannot access using indices let c = s[0];

    for c in s.chars().rev() {
        print!("{}", c);
    }
    println!("");

    if let Some(ch) = s.chars().nth(0) {
        println!("0th char: {}", ch);
    }

    // Heap allocated => String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let l: &str = &letters;

    letters.remove(letters.len() - 1);
    // Concatenation
    // String + str
    let z = letters + "!!!";
    println!("{}", z);

    let mut a = String::from("test!!!!"); // or "test".to_string()
    a.remove(a.len() - 1);
    a = a.replace("test", "passed");
    println!("{}", a);
}

fn snp(x: u32, y: u32) -> (u32, u32) {
    (x + y, x * y)
}

fn tuples() {
    println!("\nTuples:");
    let x = 3;
    let y = 4;
    let res = snp(x, y);
    println!("Result: {:?}", res);
    println!("Sum: {}, Product: {}", res.0, res.1);

    // Destructuring
    let (s, p) = res;
    println!("Sum: {}, Product: {}", s, p);
}

fn hash_maps() {
    println!("\nHash maps:");
    let mut shapes = std::collections::HashMap::new();

    // Key should have eq and hash trait implemented!!!!!!!!!!
    //let p = Point { x: 1.1, y: 1.2 };
    // shapes.insert(&p, 1); // This errs out!!!!

    shapes.insert("triangle".to_string(), 3);
    println!("{:?}", shapes);
    println!("Triangle = {}", shapes["triangle"]);

    // If not found, insert
    let val = shapes.entry("cicle".into()).or_insert(1);

    // If a value is found, a mutable reference is returned. This can be dereferenced to access the value.
    *val = 2;
    // Else
    // Entry(VacantEntry(<Key>)) is returned, which cannot be dereferenced.

    println!("{:?}", val);
}

fn main() {
    structs();
    enums();
    unions();
    options();
    array();
    vectors();
    slices();
    strings();
    tuples();
    hash_maps();
}
