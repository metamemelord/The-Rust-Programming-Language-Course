use crate::traits::{Animal, Dog, Human};

pub fn init() {
    let mut creatures: Vec<Box<dyn Animal>> = Vec::new();
    creatures.push(Box::new(Human::new("Gaurav")));
    creatures.push(Box::new(Dog::new("James")));
    println!("\nGeneric vectors:");

    for c in creatures {
        c.talk();
    }
}
