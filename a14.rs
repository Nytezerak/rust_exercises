// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person{
    name: String,
    age: i32,
    fav_color: String,
}

fn print_info(data: &String) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person{
            name: String::from("Bulbasaur"),
            age: 5,
            fav_color: "green".to_owned(),
        },
        Person{
            name: String::from("Ivysaur"),
            age: 25,
            fav_color: "blue".to_owned(),
        },
        Person{
            name: String::from("Venusaur"),
            age: 35,
            fav_color: "purple".to_owned(),
        },
    ];

    for person in people {
        if person.age >= 20 {
            print_info(&person.name);
            print_info(&person.fav_color)
        }
    }
}
