// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket{
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let unit_ticket = vec![
        Ticket::Backstage(300.00, "cleiton".to_owned()),
        Ticket::Standard(250.00),
        Ticket::Vip(150.00, "adailton".to_owned()),
    ];

    for unit in unit_ticket{
        match unit{
         Ticket::Backstage(price, holder) => {
            println!("Ticket price: {:?}\nHolder: {:?}\n", price, holder)
         }  
         Ticket::Standard(price) => {
            println!("Ticket price: {:?}\n", price)
         } 
         Ticket::Vip(price, holder) => {
            println!("Ticket price: {:?}\nHolder: {:?}\n", price, holder)
         }
        }
    }
}
