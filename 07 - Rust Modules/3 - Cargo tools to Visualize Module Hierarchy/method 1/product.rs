pub use category::Category;
pub struct Product {
    id: u64, // this can be made private
    name: String,
    price: f64,
    category: Category, // this can be made private
}
mod category;
impl Product {
    // private
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    // public function
    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}
