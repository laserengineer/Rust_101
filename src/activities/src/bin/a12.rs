// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:

// * Use an enum for the box color
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
            Color::Green => println!("Green"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!(
impl Dimensions {
    fn print(&self) {
        println!(
            "Length: {:.2}, Width: {:.2}, Height: {:.2}",
            self.length, self.width, self.height
        );
        );
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
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Box {
            dimensions,
            weight,
            color,
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
    let dimensions = Dimensions {
        length: 1.3,
        width: 2.5,
        height: 3.6,
    };

    let box1 = Box::new(dimensions, 54.7, Color::Red
        Dimensions {
            length: 5.0,
            width: 10.0,
            height: 15.0,
        },
        5.0,
        Color::Blue,
    );
    box1.print_characteristics();
}
