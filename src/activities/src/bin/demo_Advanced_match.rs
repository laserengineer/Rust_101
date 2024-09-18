// Advanced match functions with enums and structures

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("Number: {:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("Flat two"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "Concert".to_owned(),
        price: 500,
    };

    match concert {
        Ticket { price: 50, event } => println!("Event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("Price is {:?}", price),
    }
}
