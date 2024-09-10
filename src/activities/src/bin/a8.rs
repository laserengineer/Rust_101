// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
#[allow(dead_code)]

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

// * Use a function to print out the drink flavor and ounces

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("Flavor: Sparkling, Fluid Ounces: {:?}", drink.fluid_oz),
        Flavor::Sweet => println!("Flavor: Sweet, Fluid Ounces: {:.3}", drink.fluid_oz),
        Flavor::Fruity => println!("Flavor: Fruity, Fluid Ounces: {:?}", drink.fluid_oz),
    }
}

// * Use a match expression to print the drink flavor

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);
    // * Print the flavor of a drink and it's fluid ounces
}
