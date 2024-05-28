use crate::customer::Customer;
use crate::product::Product;
pub struct Order {
    id: u64, // private
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    // Private
    fn calculate_discount(&self) -> f64 {
        if self.quantity > 5 {
            0.1
        } else {
            0.0
        }
    }

    // Public
    fn total_bill(&self) -> f64 {
        let discount = self.calculate_discount();
        let total_before_discount = self.product.product_price() * self.quantity as f64;
        total_before_discount - (total_before_discount * discount)
    }
}
