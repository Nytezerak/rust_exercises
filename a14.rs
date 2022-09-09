// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of pokemon idd 10 and under
//
// Notes:
// * Use a struct for a Pokemons id, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 pokemon in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which Pokemon's info should be printed
// * The name and colors should be printed using a function
struct Pokemon{
    name: String,
    id: i32,
    mon_type: String,
}

fn print_info(data: &String) {
    println!("{:?}", data);
}

fn main() {
    let pokemon = vec![
        Pokemon{
            name: String::from("Bulbasaur"),
            id: 1,
            mon_type: "grass/poison".to_owned(),
        },
        Pokemon{
            name: String::from("Ivysaur"),
            id: 2,
            mon_type: "grass/poison".to_owned(),
        },
        Pokemon{
            name: String::from("Venusaur"),
            id: 3,
            mon_type: "grass/poison".to_owned(),
        },
    ];

    for dex_num in pokemon {
        if dex_num.id >= 2 {
            print_info(&dex_num.name);
            print_info(&dex_num.mon_type)
        }
    }
}
