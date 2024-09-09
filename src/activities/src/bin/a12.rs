// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:

use std::fmt;

// * Use an enum for the box color
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Length: {:.2}, Width: {:.2}, Height: {:.2}",
            self.length, self.width, self.height
        )
    }
}

impl Color {
    //Convert the color to a string representation
    fn to_string(&self) -> &'static str {
        match self {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
        }
    }
}

// * Use a struct to encapsulate the box characteristics
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box {
    // * Implement functionality on the box struct to create a new box
    fn new() -> Self {
        Self {
            dimensions: Dimensions {
                length: 5.0,
                width: 10.0,
                height: 15.0,
            },
            weight: 5.0,
            color: Color::Blue,
        }
    }
    // * Implement functionality on the box struct to print the characteristics
    fn print_characteristics(&self) {
        println!("Dimensions: {}", self.dimensions);
        println!("Weight: {}", self.weight);
        println!("Color: {}", self.color.to_string());
    }
}

fn main() {
    let box1 = Box::new();
    box1.print_characteristics();
}
