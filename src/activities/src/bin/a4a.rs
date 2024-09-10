// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let n = 9;

    match n {
        11 => println!("it's larger than 10"),
        9 => println!("it's smaller than 10"),
        _ => println!("It is 10"), // handle other cases
    }
}
