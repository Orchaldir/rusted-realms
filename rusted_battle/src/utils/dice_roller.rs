pub trait DiceRoller {
    fn roll(&mut self, sides: u32) -> u32;
}

// MockDiceRoller

pub struct MockDiceRoller {
    results: Vec<u32>,
    index: usize,
}

impl MockDiceRoller {
    pub fn new(results: Vec<u32>) -> MockDiceRoller {
        MockDiceRoller { results, index: 0 }
    }
}

impl DiceRoller for MockDiceRoller {
    fn roll(&mut self, _: u32) -> u32 {
        if self.index < self.results.len() {
            let result = self.results[self.index];
            self.index += 1;
            return result;
        }

        panic!("Index {} is outside the result vector!", self.index);
    }
}
