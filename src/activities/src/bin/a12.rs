// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:

#[allow(dead_code)]

// * Use an enum for the box color
enum Color {
    Red,
    Blue,
    Green,
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
    dimensions: (f64, f64, f64),
    weight: f64,
    color: Color,
}

impl Box {
    // * Implement functionality on the box struct to create a new box
    fn new() -> Self {
        Self {
            dimensions: (10.0, 15.0, 20.0),
            weight: 5.0,
            color: Color::Blue,
        }
    }
    // * Implement functionality on the box struct to print the characteristics
    fn print_characteristics(&self) {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {}", self.weight);
        println!("Color: {}", self.color.to_string());
    }
}

fn main() {
    let box1 = Box::new();
    box1.print_characteristics();
}
