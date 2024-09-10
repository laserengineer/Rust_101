// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
enum Color {
    Red,
    Yellow,
    Blue,
}

// * Use a function to print the color name

fn print_color(color: Color) {
    match color {
        Color::Red => println!("The color is red"),
        Color::Yellow => println!("The color is yellow"),
        Color::Blue => println!("The color is blue"),
    }
}

// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
    let color = Color::Red;
    print_color(color);
}
