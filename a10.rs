// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn size_checker(a: bool) {
    match a {
        true => println!("it's big"),
        false => println!("it's small"),
    }

}
fn main() {
    let size = 101;
    let check_size = size > 100;
    size_checker(check_size)
}


