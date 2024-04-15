use tokio::time::{sleep, Duration};


use crate::observer_processor::{ // Import specific components from the observer_processor module
    Order,      // Represents an order placed by a customer.
    Observer,    // Acts as the central coordinator for notifying observers about order changes.
};// Define a struct for handling payment processing.
pub struct PaymentProcessor {
    identifier: String,
}

// Implementation for PaymentProcessor.
impl PaymentProcessor {
    // Constructor for PaymentProcessor, initializing with an identifier.
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
        }
    }
}

// Implement the Observer trait for PaymentProcessor.
impl Observer for PaymentProcessor {
    // Method to handle notification logic for PaymentProcessor.
    fn notify(&self, order: &Order) -> tokio::task::JoinHandle<()> {
        let id = order.order_id;
        let product = order.product_name.clone();
        let quantity = order.quantity;
        let identifier = self.identifier.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(3)).await;  // Simulate a 3-second payment process
            println!("{} finished processing payment for Order {}: {} x {}",
                     identifier, id, product, quantity);
        })
    }
    fn is_interested(&self, _order: &Order) -> bool {
        true // Always interested in processing payments for any order.
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}