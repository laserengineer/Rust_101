// use string for LineItem type and string f

struct LineItem {
    name: String, // require owned string instead of borrowed string
    count: i32,
}

fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: "milk".to_owned(),
            count: 2,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for item in receipt.iter() {
        print_name(&item.name); // use print_name function instead of println! to avoid borrowing item.name
        println!("count: {:?}", item.count);
    }
}
