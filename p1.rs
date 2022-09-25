// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::io;

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please re-enter your data");
    }
    let input = buffer.trim().to_owned();
    if let &input = "" {
        None
    }else {
        Some(input)
    }
}

enum MainMenu {
    Add,
    View,
    Remove,
    Update,
    Total,
}

impl MainMenu {
    fn input_string (input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::Add),
            "2" => Some(Self::View),
            "3" => Some(Self::Remove),
            "4" => Some(Self::Update),
            "5" => Some(Self::Total),
            _ => None,
        }
    }
    fn show_menu() {
        println!("\n=-=HELP BILL=-=\n
                1> Add bill\n
                2> View bills\n
                3> Remove bill\n
                4> Update bill\n
                5> Bill total\n\n
                Enter selection: ");
    }
}
fn main() {
    println!("Welcome! Please select an option below");
    loop {
        MainMenu::show_menu();
        let input = get_input().expect("no data entered");
        match MainMenu::input_string(input.as_str()) {
            Some(MainMenu::Add) => (),
            Some(MainMenu::View) => (),
            Some(MainMenu::Remove) => (),
            Some(MainMenu::Update) => (),
            Some(MainMenu::Total) => (),
            None => return,
        };
    }
}
