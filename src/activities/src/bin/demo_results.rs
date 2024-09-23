// use Demo Results
// Simulate a menu enumeration with multiple choices and use is able to enter some of the text that corresponds to one of the choices
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input.to_lowercase().as_str() {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice)
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?; // if an error happened during the get_choice() call, return error will quit the function
    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice = pick_choice("Quit");
    println!("choice value = {:?}", choice);
}
