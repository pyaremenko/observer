trait Observer {
    fn notify(&self, data: &str);
}

struct ConcreteObserver {
    name: String,
}

impl ConcreteObserver {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Observer for ConcreteObserver {
    fn notify(&self, data: &str) {
        println!("{} received data: {}", self.name, data);
    }
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify_observers(&self, data: &str) {
        for observer in &self.observers {
            observer.notify(data);
        }
    }
}

fn main() {
    let mut subject = Subject::new();

    let observer1 = Box::new(ConcreteObserver::new("Observer 1"));
    let observer2 = Box::new(ConcreteObserver::new("Observer 2"));

    subject.add_observer(observer1);
    subject.add_observer(observer2);

    subject.notify_observers("Hello, observers!");
}
