const PI: f64 = std::f64::consts::PI;

use std::fmt::Debug;

pub fn init() {
    println!("\nTraits:");

    let h = Human { name: "Gaurav" };
    h.talk();

    let d = Dog { name: "John" };
    d.talk();

    // Type determined at runtime
    let doggo: Dog = Animal::new("Pupper");

    let a = vec![1, 4, 9];
    println!("Sum of {:?} is {}", a, a.sum());

    let c = Circle { radius: 1f64 / PI };
    println!("Perimeter of circle is {}", c.perimeter());

    // Trait param
    // make_animal_talk(doggo);
    // make_animal_talk_trait_bound_syntax(doggo);
    make_animal_talk_with_where(doggo);

    // INTO
    let john = Person::new("John");

    let name = "Gaurav".to_string();
    // have to make it &str by using name.as_ref()
    let gaurav = Person::new(name.as_ref());
    let another_gaurav = Person::new_with_into("gaurav");

    let ch = Creature::new("Creature");
    drop(ch);
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk!", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn new(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("This is {} talking!!!", self.name);
    }
}

#[derive(Debug)]
struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} can only bark; Woof!!!", self.name);
    }
}

fn make_animal_talk(animal: impl Animal + Debug) {
    print!("Here is {:?} talking: ", animal);
    animal.talk();
}

fn make_animal_talk_trait_bound_syntax<T: Animal + Debug>(animal: T) {
    print!("Here is {:?} talking: ", animal);
    animal.talk();
}

fn make_animal_talk_with_where<T>(animal: T)
where
    T: Animal + Debug,
{
    print!("Here is {:?} talking: ", animal);
    animal.talk();
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        self.iter().fold(0, |s, x| s + x)
    }
}

trait Shape<T> {
    fn area(&self) -> T;
    fn perimeter(&self) -> T;
}

struct Circle {
    radius: f64,
}

impl Shape<f64> for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
    fn perimeter(&self) -> f64 {
        2f64 * PI * self.radius
    }
}

struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
    fn new_with_into<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
    fn new_with_where<S>(name: S) -> Person
    where
        S: Into<String>,
    {
        Person { name: name.into() }
    }
}

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} has died", self.name);
    }
}
