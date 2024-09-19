// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
    // * Use an enum for the tickets with data associated with each variant
    enum Ticket {
        Backstage(f64, String),
        Standard(f64),
        Vip(f64, String),
    }

    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(15.0, "Alice".to_owned()),
        Ticket::Standard(10.0),
        Ticket::Vip(20.0, "Bob".to_string()),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket holder： {}: ${:.2}", holder, price);
            }
            Ticket::Standard(price) => {
                println!("Standard ticket: ${:.2}", price);
            }
            Ticket::Vip(price, holder) => {
                println!("VIP ticket holder： {}: ${:.2}", holder, price);
            }
        }
    }
}
