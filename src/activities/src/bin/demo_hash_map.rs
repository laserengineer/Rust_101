// Demo for hash map for student lockers
// Each locker will have a number and items in it

use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers: HashMap<i32, Contents> = HashMap::new();

    lockers.insert(
        1,
        Contents {
            content: "book".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "shirt".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "stuff".to_owned(),
        },
    );

    for (locker_id, item) in lockers.iter() {
        println!("Locker number {:?}: {:?}", locker_id, item.content);
    }
}
