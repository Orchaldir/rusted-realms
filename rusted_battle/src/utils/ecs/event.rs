use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub trait EventListener<T> {
    fn update(&mut self, event: &T);
}

pub struct EventPublisher<T> {
    listeners: Vec<Rc<RefCell<dyn EventListener<T>>>>,
}

impl<T> EventPublisher<T> {
    pub fn new() -> EventPublisher<T> {
        EventPublisher {
            listeners: Vec::new(),
        }
    }

    pub fn add(&mut self, listener: Rc<RefCell<dyn EventListener<T>>>) {
        self.listeners.push(listener);
    }

    pub fn publish(&self, event: &T) {
        for l in self.listeners.iter() {
            let cell: &RefCell<dyn EventListener<T>> = l.borrow();
            (*cell.borrow_mut()).update(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct MockEvent {
        pub data: u32,
    }

    pub struct MockListener {
        pub data: u32,
    }

    impl EventListener<MockEvent> for MockListener {
        fn update(&mut self, event: &MockEvent) {
            self.data = event.data;
        }
    }

    #[test]
    fn test_add() {
        let event = MockEvent { data: 55 };
        let listener = Rc::new(RefCell::new(MockListener { data: 0 }));

        let mut publisher = EventPublisher::new();
        publisher.add(listener.clone());

        publisher.publish(&event);

        let cell: &RefCell<MockListener> = listener.borrow();
        let l = cell.borrow().data;
        assert_eq!(l, 55);
    }
}
