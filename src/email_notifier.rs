use tokio::time::{sleep, Duration};


use crate::observer_processor::{ // Import specific components from the observer_processor module
    Order,      // Represents an order placed by a customer.
    Observer,    // Acts as the central coordinator for notifying observers about order changes.
};// Define a struct for handling email notifications.
pub struct EmailNotifier {
    identifier: String,
}

// Implementation for EmailNotifier.
impl EmailNotifier {
    // Constructor for EmailNotifier, initializing with an identifier.
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
        }
    }
}

// Implement the Observer trait for EmailNotifier.
impl Observer for EmailNotifier {
    // Method to handle notification logic for EmailNotifier.
    fn notify(&self, order: &Order) -> tokio::task::JoinHandle<()> {
        let id = order.order_id;
        let product = order.product_name.clone();
        let identifier = self.identifier.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(1)).await;  // Simulate email sending delay
            println!("{} sent email confirmation for Order {}: {}", identifier, id, product);
        })
    }
    fn is_interested(&self, order: &Order) -> bool {
        order.quantity >= 5 // Only interested in sending emails for orders with at least 5 items.
    }
    
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}