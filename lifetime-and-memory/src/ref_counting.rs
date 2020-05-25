#![allow(unused_imports)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn init() {
    println!("\nReference Counting and Mutex:");
    // let value = String::from("some value:");
    // let value_rc = Rc::new(String::from("some value:"));
    let value_arc = Arc::new(String::from("some value"));
    let se = SmallExample {
        value: value_arc.clone(),
    };

    let t = thread::spawn(move || println!("Value from thread is {{{}}}", se.value));
    t.join().unwrap();

    let another_value_arc = Arc::new(Mutex::new(String::from("Some other value")));
    let ase = AnotherSmallExample {
        value: another_value_arc,
    };

    let value_from_arc = ase.value.clone();
    let another_t = thread::spawn(move || {
        reset_value(&ase);
        println!(
            "Value of struct in closure is {{{}}}",
            ase.value.lock().unwrap()
        )
    });
    println!("Value of struct is {{{}}}", value_from_arc.lock().unwrap());
    another_t.join().unwrap();
}

struct SmallExample {
    value: Arc<String>,
}

struct AnotherSmallExample {
    value: Arc<Mutex<String>>,
}

fn reset_value(ase: &AnotherSmallExample) {
    let mut value = ase.value.lock().unwrap();
    value.clear();
    value.push_str("Another value");
}
