// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_id(beans:&GroceryItem){
    println!("Id: {:?}", beans.id);
}

fn display_quantity(beans:&GroceryItem){
    println!("Quantity: {:?}", beans.quantity)
}

fn main() {
    let beans = GroceryItem{
        quantity: 15,
        id: 01
    };
    display_id(&beans);
    display_quantity(&beans);
}
