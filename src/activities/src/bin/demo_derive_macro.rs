// use of derive debug information macros

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Supervisor,
        work_hours: 40,
    };
    println!("{:?}", me.position);
    println!("{:?}", me.work_hours);
    // println!("{:?}", me); // prints debug information
    print_employee(me); // prints debug information too, but not the struct itself.
}
