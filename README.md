## Order Processing System with Observer Pattern

This project demonstrates an order processing system using the Observer pattern in Rust. It leverages asynchronous programming for efficient handling of independent tasks.

**Observer Pattern:**

The Observer pattern is a design pattern that allows objects to define a subscription mechanism for receiving notifications about changes in another object (the subject). When the subject's state changes, it notifies all its registered observers, allowing them to update or react accordingly. This pattern promotes loose coupling between objects and facilitates modular development.

**Project Structure:**

The code defines several structs:

* `Order`: Represents an order with details like ID, product name, and quantity.
* `PaymentProcessor`, `PaymentGateway`, `EmailNotifier`, and `LogisticsManager`: Implement the `Observer` trait, which defines a `notify` method to handle notifications for payment, payment verification, email confirmation, and logistics planning, respectively.
* `InventoryManager`: Also implements `Observer` to update inventory levels for placed orders.
* `Subject`: Manages a list of registered observers and provides methods to add and notify them about new orders.

**Functionality:**

* `Subject` instances maintain a list of observers.
* Observers implement the `Observer` trait, requiring a `notify` method that performs actions upon receiving notifications.
* The `place_order` method of `Subject` asynchronously notifies all registered observers about a new order, triggering their respective `notify` methods.

**Running the Project:**

(Instructions assuming Rust and Cargo are already installed)

1. Clone the repository
   ```bash
   git clone https://github.com/pyaremenko/observer.git
   ```

2. Navigate to the project directory.
   ```bash
   cd observer
   ```

3. Build and run the project.
   ```bash
   cargo run
   ```

This will simulate order processing, showcasing how each observer reacts to a new order.

**Further Exploration:**

* Implement additional logic within the `notify` methods of observers for more complex processing steps.
* Explore error handling mechanisms to gracefully handle potential issues.
* Integrate this code with a web framework (e.g., Actix Web, Rocket) to create a full-fledged order processing application.
