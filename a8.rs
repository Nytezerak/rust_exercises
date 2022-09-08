// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor{
    _Cola,
    Grape,
    Orange
}

struct Drink {
    flavor: Flavor,
    ml: i32,
}

fn print_drink(drink: Drink){
     match drink.flavor{
        Flavor::Orange => println!("Flavor: Orange!"),
        Flavor::_Cola => println!("Flavor: Cola!"),
        Flavor::Grape => println!("Flavor: Grape!"),
     }

     println!("ml: {:?}", drink.ml);
}
fn main() {
    let grape = Drink{
        flavor: Flavor::Grape,
        ml: 365,
    };
    print_drink(grape);

    let orange = Drink{
        flavor: Flavor::Orange,
        ml: 600,
    };
    print_drink(orange)

}
