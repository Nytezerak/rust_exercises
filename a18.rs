// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct CustomerInfo{
    age: i32,
}

fn restricted_purchase(customer: &CustomerInfo) -> Result<String, String>{
    if customer.age < 18 {
        Err("I'm sorry, you are too young".to_owned())
    }else{
        Ok("Have a good day!".to_owned())
    }
}

fn main() {
    let cleitin = CustomerInfo{
        age: 19,
    };

    let purchase = restricted_purchase(&cleitin);
    println!("{:?}", purchase)
}
