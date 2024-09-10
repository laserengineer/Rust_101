// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn coordinate() -> (i32, i32) {
    (1, 7)
}

// * Use an if..else if..else block to determine what to print

fn main() {
    // * Destructure the return value into two variables
    let (x, y) = coordinate();
    // * Use an if..else if..else block to determine what to print
    // * Use a match expression to determine which message to print

    match y {
        y if y > 5 => println!("Y is greater than 5"),
        y if y < 5 => println!("Y is less than 5"),
        _ => println!("Y is equal to 5"),
    }
}
