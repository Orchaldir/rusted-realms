use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

pub trait EventListener<T> {
    fn update(&mut self, event: &T);
}

pub struct EventManager<T> {
    listeners: Vec<Rc<RefCell<dyn EventListener<T>>>>,
}

impl<T> EventManager<T> {
    pub fn new() -> EventManager<T> {
        EventManager {
            listeners: Vec::new(),
        }
    }

    pub fn add(&mut self, listener: Rc<RefCell<dyn EventListener<T>>>) {
        self.listeners.push(listener);
    }

    pub fn publish(&mut self, event: &T) {
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

        let mut manager = EventManager::new();
        manager.add(listener.clone());

        manager.publish(&event);

        let cell: &RefCell<MockListener> = listener.borrow();
        let l = cell.borrow().data;
        assert_eq!(l, 55);
    }
}
