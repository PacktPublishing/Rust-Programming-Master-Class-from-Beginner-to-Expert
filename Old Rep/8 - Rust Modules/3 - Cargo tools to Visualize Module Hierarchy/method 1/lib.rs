// cargo install cargo-modules
// the commands may not work on the older version
// cargo module generate tree
//cargo module generate tree --lib (only modules)
// cargo modules generate tree --lib --types --fns   (details)
mod customer;
mod order;
mod product;

/*
// products.rs

pub mod products {
    pub struct Product {
        pub id: u64,
        pub name: String,
        price: f64,
    }

    impl Product {
        pub fn new(id: u64, name: String, price: f64) -> Product {
            Product { id, name, price }
        }

        // Private function for internal calculations
        fn calculate_tax(&self) -> f64 {
            // Perform tax calculation based on product price (private)
            self.price * 0.1 // 10% tax
        }

        // Public method to get the total price including tax
        pub fn get_total_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }
}

// customers.rs
pub mod customers {
    pub struct Customer {
        pub id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}

// orders.rs
pub mod orders {
    pub struct Order {
        pub id: u64,
        pub product: crate::products::Product, //// change to crate::products::Product
        pub customer: crate::customers::Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(
            id: u64,
            product: crate::products::Product,
            customer: crate::customers::Customer,
            quantity: u32,
        ) -> Order {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }

        // Private function for internal calculations
        fn calculate_discount(&self) -> f64 {
            // Perform discount calculation based on quantity (private)
            if self.quantity > 5 {
                0.1 // 10% discount for orders with more than 5 items
            } else {
                0.0
            }
        }

        // Public method to calculate order total (including discount)
        pub fn calculate_total(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.get_total_price() * f64::from(self.quantity);
            total_before_discount - (total_before_discount * discount)
        }
    }
}
*/
