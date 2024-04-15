use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::observer_processor::{ // Import specific components from the observer_processor module
    Order,      // Represents an order placed by a customer.
    Observer,   // Interface for objects that can be notified about order changes.
    Subject,    // Acts as the central coordinator for notifying observers about order changes.
};// Define a struct for managing inventory updates.
pub struct InventoryManager {
    identifier: String,
}

// Implementation for InventoryManager.
impl InventoryManager {
    // Constructor for InventoryManager, initializing with an identifier.
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
        }
    }
}

// Implement the Observer trait for InventoryManager.
impl Observer for InventoryManager {
    // Method to handle notification logic for InventoryManager.
    fn notify(&self, order: &Order) -> tokio::task::JoinHandle<()> {
        let id = order.order_id;
        let product = order.product_name.clone();
        let quantity = order.quantity;
        let identifier = self.identifier.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;  // Simulate a 2-second inventory update
            println!("{} updated inventory for Order {}: {} x {}",
                     identifier, id, product, quantity);
        })
    }
    
    fn is_interested(&self, order: &Order) -> bool {
        false
    }
    
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}