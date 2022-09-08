// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    _Red,
    _Blue,
    White,
    _Green,
}

fn print_color(my_color: Color){
    match my_color{
        Color::_Red => println!("red"),
        Color::_Blue => println!("blue"),
        Color::White => println!("white"),
        Color::_Green => println!("green"),
    };
}

fn main() {
    print_color(Color::White)    
}