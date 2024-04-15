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
