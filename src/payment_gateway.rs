use tokio::time::{sleep, Duration};


use crate::observer_processor::{ // Import specific components from the observer_processor module
    Order,      // Represents an order placed by a customer.
    Observer,    // Acts as the central coordinator for notifying observers about order changes.
};// Define a struct for handling payment verification through a gateway.
pub struct PaymentGateway {
    identifier: String,
}

// Implementation for PaymentGateway.
impl PaymentGateway {
    // Constructor for PaymentGateway, initializing with an identifier.
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
        }
    }
}

// Implement the Observer trait for PaymentGateway.
impl Observer for PaymentGateway {
    // Method to handle notification logic for PaymentGateway.
    fn notify(&self, order: &Order) -> tokio::task::JoinHandle<()> {
        let id = order.order_id;
        let product = order.product_name.clone();
        let quantity = order.quantity;
        let identifier = self.identifier.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;  // Simulate a 1-second verification time
            println!("{} verified payment details for Order {}: {} x {}",
                     identifier, id, product, quantity);
        })
    }
    
    fn is_interested(&self, _order: &Order) -> bool {
        false
    }
    
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}