use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;

// Define a trait for Observer which requires a notify method.
pub trait Observer {
    // Notify method which takes an order reference and returns a join handle.
    fn notify(&self, order: &Order) -> tokio::task::JoinHandle<()>;
    fn is_interested(&self, order: &Order) -> bool;
    fn get_identifier(&self) -> &str;
}

// Define a struct for Order with basic order details.
#[derive(Clone)]
pub struct Order {
    pub order_id: u32,
    pub product_name: String,
    pub quantity: u32,
}

// Implementation of the Order struct.
impl Order {
    // Constructor for Order, initializing with order_id, product_name, and quantity.
    pub fn new(order_id: u32, product_name: &str, quantity: u32) -> Self {
        Self {
            order_id,
            product_name: product_name.to_string(),
            quantity,
        }
    }
}

// Define a struct for handling payment processing.
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

// Define a struct for handling payment verification through a gateway.
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
    
    fn is_interested(&self, order: &Order) -> bool {
        false
    }
    
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

// Define a struct for handling email notifications.
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

// Define a struct for managing logistics operations.
pub struct LogisticsManager {
    identifier: String,
}

// Implementation for LogisticsManager.
impl LogisticsManager {
    // Constructor for LogisticsManager, initializing with an identifier.
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
        }
    }
}

// Implement the Observer trait for LogisticsManager.
impl Observer for LogisticsManager {
    // Method to handle notification logic for LogisticsManager.
    fn notify(&self, order: &Order) -> tokio::task::JoinHandle<()> {
        let id = order.order_id;
        let product = order.product_name.clone();
        let quantity = order.quantity;
        let identifier = self.identifier.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(2)).await;  // Simulate logistics planning delay
            println!("{} arranged shipping for Order {}: {} x {}", identifier, id, product, quantity);
        })
    }
    
    fn is_interested(&self, order: &Order) -> bool {
        false
    }
    
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

// Define a struct for managing inventory updates.
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

// Define a class to manage subjects and their observers.
pub struct Subject {
    observers: Vec<Arc<dyn Observer + Send + Sync>>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Arc<dyn Observer + Send + Sync>) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, identifier: &str) {
        self.observers.retain(|observer| observer.get_identifier() != identifier);
    }

    pub async fn notify_observers(&self, order: &Order) {
        let mut handles = vec![];
        for observer in &self.observers {
            if observer.is_interested(order) {
                handles.push(observer.notify(order));
            }
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
 


    pub async fn place_order(&mut self, order: Order) {
        self.notify_observers(&order).await;
    }
}
