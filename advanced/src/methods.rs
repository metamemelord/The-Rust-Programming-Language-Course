struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(st: Point, en: Point) -> Self {
        Line { start: st, end: en }
    }

    fn to_string(&self) -> String {
        format!(
            "Line: ({},{}) -> ({},{})",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }

    fn len(&self) -> f64 {
        ((self.start.x - self.end.x) * (self.start.x - self.end.x)
            + (self.start.y - self.end.y) * (self.start.y - self.end.y))
            .sqrt()
    }
}

pub fn init() {
    let st = Point::new(0f64, 0f64);
    let en = Point::new(1f64, 0f64);
    let line = Line::new(st, en);
    println!("{} has length {:.2}", line.to_string(), line.len());
}
