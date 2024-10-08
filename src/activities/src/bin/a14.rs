// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String

struct Person {
    name: String,
    fav_color: String,
    age: i32,
}
// * The name and colors should be printed using a function
fn print_person_info(person: &Person) {
    println!(
        "Name: {}, Favorite Color: {}",
        person.name, person.fav_color
    );
}

fn main() {
    let people = vec![
        Person {
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 7,
        },
        Person {
            name: String::from("Anna"),
            fav_color: String::from("purple"),
            age: 9,
        },
        Person {
            name: String::from("Katie"),
            fav_color: String::from("blue"),
            age: 14,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print_person_info(&person);
        }
    }
}

// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print_person(data: &str) {
    println!("{:?}", &data);
}

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("John"),
            fav_color: String::from("Blue"),
            age: 2,
        },
        Person {
            name: "Sarah".to_owned(),
            fav_color: "Green".to_owned(),
            age: 7,
        },
        Person {
            name: "Mike".to_owned(),
            fav_color: "Red".to_owned(),
            age: 10,
        },
    ];
    // * Use an if expression to determine which person's info should be printed
    for person in &people {
        if person.age <= 10 {
            print_person(&person.name);
            print_person(&person.fav_color);
        }
    }
}
