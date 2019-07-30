use std::cell::Cell;

pub trait DiceRoller {
    fn roll(&self, sides: u32) -> u32;
}

// MockDiceRoller

pub struct MockDiceRoller {
    results: Vec<u32>,
    index: Cell<usize>,
}

impl MockDiceRoller {
    pub fn new(results: Vec<u32>) -> MockDiceRoller {
        MockDiceRoller {
            results,
            index: Cell::new(0),
        }
    }
}

impl DiceRoller for MockDiceRoller {
    fn roll(&self, _: u32) -> u32 {
        let current_index = self.index.get();

        if current_index < self.results.len() {
            let result = self.results[current_index];
            self.index.set(current_index + 1);
            return result;
        }

        panic!("Index {} is outside the result vector!", current_index);
    }
}
