use std::collections::HashMap;

pub struct ComponentManager<T> {
    components: HashMap<u32, T>,
}

impl<T> ComponentManager<T> {
    pub fn new() -> ComponentManager<T> {
        ComponentManager {
            components: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: u32, component: T) {
        self.components.insert(entity, component);
    }

    pub fn remove(&mut self, entity: u32) {
        self.components.remove(&entity);
    }

    pub fn get(&self, entity: u32) -> Option<&T> {
        self.components.get(&entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let id: u32 = 0;
        let text = "test1";
        let component = String::from(text);
        let mut manager = ComponentManager::new();

        manager.add(id, component);

        assert_eq!(manager.get(id).unwrap(), text)
    }

    #[test]
    fn test_remove() {
        let id: u32 = 0;
        let text = "test1";
        let component = String::from(text);
        let mut manager = ComponentManager::new();

        manager.add(id, component);
        manager.remove(id);

        assert_eq!(manager.get(id), None)
    }

    #[test]
    fn test_overwrite() {
        let id: u32 = 0;
        let text_a = "test_a";
        let text_b = "test_b";
        let component_a = String::from(text_a);
        let component_b = String::from(text_b);
        let mut manager = ComponentManager::new();

        manager.add(id, component_a);
        manager.add(id, component_b);

        assert_eq!(manager.get(id).unwrap(), text_b)
    }
}
