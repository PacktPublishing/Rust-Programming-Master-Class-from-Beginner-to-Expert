use testing::{Category, Customer, Order, Product};
mod helpers;
#[test]
fn test_total_bill_without_discount() {
    helpers::common_setup();
    let product = Product::new(1, String::from("Book"), 19.9, Category::Books);
    let customer = Customer::new(1, String::from("Bob"), String::from("bob@example.com"));
    let order = Order::new(2, product, customer, 3);

    assert_eq!(format!("{:.2}", order.total_bill()), "65.67");
}

#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, String::from("Book"), 19.99, Category::Books);
    let customer = Customer::new(1, String::from("Bob"), String::from("bob@example.com"));
    let order = Order::new(2, product, customer, 10); // change to 10 later on
    assert_eq!(format!("{:.2}", order.total_bill()), "197.90"); // change to 197.90
}


// store in a separate file inside the tests/helpers/mod.rs 
pub fn common_setup();