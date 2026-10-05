//! Runs the experiments that are worth seeing the output of.
//!
//! The assertions live in the domain crate as `#[test]` functions; this binary
//! is for the ones where watching the value is the point.

use domain::errors::{self, ParseAgeError};
use domain::iterators;
use domain::patterns::Shape;
use domain::random;
use domain::serialization::{self, Experiment};
use domain::traits::{English, Greeter, Pirate};

fn main() {
    section("patterns");
    for shape in [
        Shape::Point,
        Shape::Circle { radius: 2.0 },
        Shape::Rectangle { width: 3.0, height: 3.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ] {
        println!("{:>24}  area {:.4}", shape.describe(), shape.area());
    }

    section("traits");
    let greeters: Vec<Box<dyn Greeter>> =
        vec![Box::new(English { who: "world".to_string() }), Box::new(Pirate)];
    for greeter in &greeters {
        println!("{}", greeter.greet());
    }

    section("iterators");
    println!("even squares to 10: {:?}", iterators::even_squares(10));
    println!("first 10 fibonacci: {:?}", iterators::first_fibonacci(10));

    section("errors");
    for input in ["42", "abc", "900"] {
        match errors::parse_age(input) {
            Ok(age) => println!("{input:>5} -> age {age}"),
            Err(ParseAgeError::NotANumber(text)) => println!("{text:>5} -> not a number"),
            Err(error) => println!("{input:>5} -> {error}"),
        }
    }

    section("serialization");
    let experiment = Experiment {
        name: "ownership".to_string(),
        iterations: 3,
        complete: true,
        notes: vec!["borrow checker".to_string()],
    };
    match serialization::to_json(&experiment) {
        Ok(json) => println!("{json}"),
        Err(error) => println!("could not serialize: {error}"),
    }

    section("random");
    println!("dice: {:?}", random::roll_dice(5));
    println!("unit interval: {:.6}", random::unit_interval());
}

fn section(title: &str) {
    println!();
    println!("=== {title} ===");
}
