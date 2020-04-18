#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

fn main() {
    let number = rand::thread_rng().gen_range(1, 501);
    loop {
        println!("Guess a number: ");
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim_end().parse::<i64>() {
                Ok(num) => {
                    if num < 1 || num > 500 {
                        println!("Well that's way out of the range friend!");
                    } else if num < number {
                        println!("Awww, that's too small!");
                    } else if num > number {
                        println!("It's sooooo big!");
                    } else {
                        println!("CORRECT, NOICE!!!");
                        break;
                    }
                }
                Err(err) => println!("Something went wrong while parsing: {}", err),
            },
            Err(err) => println!("{}", err),
        }
    }
}
