struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn to_string(self) -> String {
        format!("Point {{x: {}, y: {}}}", self.x, self.y)
    }
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn to_string(self) -> String {
        format!("Line {{start: {}, end: {}}}", self.start.to_string(), self.end.to_string())
    }
}

fn structs() {
    let p = Point{x: 3.0, y: 3.0};
    let p2 = Point{x: 5.2, y: 1.0};

    // Errors out as the value is moved here!!!!!!!!!!
    // println!("Line: {} -> {}", p.to_string(), p2.to_string());

    let line = Line{start: p, end: p2};
    println!("{}", line.to_string());
}

enum Color {
    Red,
    Green,  
    Blue,
    RGBColor(u8, u8, u8), // Tuple
    CMYKColor{cyan:u8, magenta:u8, yellow: u8, black: u8} // Struct
}

fn enums() {
    let c:Color = Color::CMYKColor{cyan: 2, magenta: 25, yellow: 41, black: 200};

    match c {
        Color::Red => println!("RED"),
        Color::Green => println!("GREEN"),
        Color::Blue => println!("BLUE"),
        Color::RGBColor(0, 0, 0) 
        | Color::CMYKColor{cyan:_, magenta: _, yellow: _, black: 255} => println!("BLACK"),
        Color::RGBColor(r, g, b) => println!("rbg({},{},{})", r, g, b),
        Color::CMYKColor{cyan:c, magenta: m, yellow: y, black: k} => println!("cmyk({},{},{},{})", c, m, y, k)
    }
}

fn main() {
    structs();
    enums();
}