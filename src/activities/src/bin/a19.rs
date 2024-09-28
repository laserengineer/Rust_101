// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

#[derive(Debug)]
struct Furniture {
    name: String,
    quantity: i32,
}

fn main() {
    let mut stock = HashMap::new();
    // * The store has:
    //   * 5 Chairs
    //   * 3 Beds
    //   * 2 Tables
    //   * 0 Couches
    stock.insert(
        "Chair",
        Furniture {
            name: "Chair".to_owned(),
            quantity: 5,
        },
    );
    stock.insert(
        "Bed",
        Furniture {
            name: "Bed".to_owned(),
            quantity: 3,
        },
    );
    stock.insert(
        "Table",
        Furniture {
            name: "Table".to_owned(),
            quantity: 2,
        },
    );
    stock.insert(
        "Couch",
        Furniture {
            name: "Couch".to_owned(),
            quantity: 0,
        },
    );

    let mut total_stock = 0;
    for (name, furniture) in stock.iter() {
        // * Print the name and number of items in stock for a furniture store
        match furniture.quantity {
            // print "out of stock" instead of 0
            0 => println!("{}: out of stock", name),
            _ => println!("{}: {:?}", furniture.name, furniture.quantity),
        }
        total_stock += furniture.quantity;
    }
    println!("Total stock: {}", total_stock);
}
