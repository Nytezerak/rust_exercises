use std::collections::{HashMap};
// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

fn main() {
    let mut items = HashMap::new();
    items.insert(5, "Chair");
    items.insert(3, "Bed");
    items.insert(2, "Table");
    items.insert(0, "Couch");

    let mut total_stock = 0;
    
    for (qty, item) in items.iter(){
        total_stock = total_stock+qty;

        let stock_count = if qty == &0 {
            "out of stock".to_owned()
        }else {
            format!("{:?}", qty)
        };
        println!("item: {:?}, stock: {:?}", item, stock_count)
    }
    println!("total stock: {:?}", total_stock);
}
