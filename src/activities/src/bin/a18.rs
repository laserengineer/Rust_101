// Topic: Result
//
// Requirements:

// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    // the person's name and age
    name: String,
    age: u8,
}

impl Adult {
    // * Implement a `new` function for the `Adult` structure that returns a Result:
    //   * The Ok variant should contain the initialized structure, but only
    //     if the person is aged 21 or older
    //   * The Err variant should contain a String (or &str) that explains why
    //     the structure could not be created
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                name: name.to_string(),
                age,
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be aged under 21
    //   * One should be 21 or over
    let child = Adult::new(15, "John Doe");

    let adult = Adult::new(21, "Jane Smith");

    // * Use `match` to print out a message for each `Adult`:
    //   * For the Ok variant, print any message you want
    //   * For the Err variant, print out the error message
    match child {
        Ok(child) => println!("Name: {} is {} years old", child.name, child.age),
        Err(err) => println!("{}", err),
    }
    match adult {
        Ok(a) => println!("Name: {} is {} years old", a.name, a.age),
        Err(err) => println!("{}", err),
    }
}
