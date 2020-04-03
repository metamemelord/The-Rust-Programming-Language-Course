#![allow(dead_code)]
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
    println!("{}", line.to_string());
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
    println!("{}", c.to_string());
}

union IoF {
    i: i32,
    f: f32,
}

fn unions() {
    let iof = IoF { i: 10 };

    // Need to be put in an unsafe block.
    unsafe {
        println!("{}", iof.i);
    }
}

fn main() {
    structs();
    enums();
    unions();
}
