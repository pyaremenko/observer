mod observer_processor; // This module contains the Observer pattern implementation for order processing.
mod email_notifier;
mod logistics_manager;
mod payment_gateway;
mod payment_processor;
mod inventory_manager;

use std::sync::Arc; // Arc (Atomically Reference Counted) is used for thread-safe shared ownership.
use crate::observer_processor::{ // Import specific components from the observer_processor module
    Order,   // Interface for objects that can be notified about order changes.
    Subject,    // Acts as the central coordinator for notifying observers about order changes.
};
use crate::payment_gateway::{
    PaymentGateway,  // Interacts with the payment gateway for authorization and capture.
};
use crate::logistics_manager::{
    LogisticsManager, // Handles logistics tasks related to orders.
};
use crate::payment_processor::{PaymentProcessor, // Processes payments for orders.
};
use crate::email_notifier::{
    EmailNotifier,  // Sends email notifications about order status.
};
use crate::inventory_manager::{InventoryManager, // Manages inventory levels for products.
};




#[tokio::main]
async fn main() {
    // Create a new Subject instance to manage order notifications.
    let mut subject = Subject::new();

    // Create thread-safe shared instances of order processing components using Arc.
    let payment_processor = Arc::new(PaymentProcessor::new("PaymentProcessor"));
    let payment_gateway = Arc::new(PaymentGateway::new("PaymentGateway"));
    let inventory_manager = Arc::new(InventoryManager::new("InventoryManager"));
    let email_notifier = Arc::new(EmailNotifier::new("EmailNotifier"));
    let logistics_manager = Arc::new(LogisticsManager::new("LogisticsManager"));

    // Register all observers with the Subject for order notifications.
    subject.add_observer(payment_processor.clone()); // Clone Arc to avoid ownership issues
    subject.add_observer(payment_gateway.clone());
    subject.add_observer(inventory_manager.clone());
    subject.add_observer(email_notifier.clone());
    subject.add_observer(logistics_manager.clone());

    // Simulate multiple orders
    let orders = vec![
        Order::new(101, "Super Widget", 10),
        Order::new(102, "Ultra Gadget", 5),
        Order::new(103, "Mega Toolset", 3),
    ];

    for order in orders {
        // Place each order asynchronously using await within the loop.
        subject.place_order(order.clone()).await;
        println!("Completed processing for Order {}", order.order_id);
    }
}
