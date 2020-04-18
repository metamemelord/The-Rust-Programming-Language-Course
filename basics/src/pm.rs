fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        6 => "half a dozen",
        many_range_name @ 4..=5 => "a lot",
        _ if x % 2 == 0 => "even number of",
        _ => "some",
    }
}

fn where_is_the_point(x: i32, y: i32) {
    match (x, y) {
        (0, 0) => println!("origin"),
        (0, y) => println!("On x-axis at y={}", y),
        (x, 0) => println!("On y-axis at x={}", x),
        (x, y) => println!("On the plane at ({},{})", x, y),
    }
}

pub fn pattern_matching() {
    println!("\nPattern matching:");
    for x in 0..7 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    where_is_the_point(0, 0);
    where_is_the_point(0, 3);
    where_is_the_point(2, 0);
    where_is_the_point(3, 5);
}
