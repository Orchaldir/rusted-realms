use std::cell::Cell;

pub trait DiceRoller {
    fn roll(&self, sides: u32) -> u32;

    fn roll_die_minus_die(&self, sides: u32) -> i32 {
        let positive_die = self.roll(sides) as i32;
        let negative_die = self.roll(sides) as i32;
        let result = positive_die - negative_die;

        println!(
            "Roll d{0}-d{0}={1}-{2}={3}",
            sides, positive_die, negative_die, result
        );

        result
    }
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

    pub fn for_two_rolls(positive_die: u32, negative_die: u32) -> MockDiceRoller {
        MockDiceRoller::new(vec![positive_die, negative_die])
    }

    pub fn for_die_minus_die(result: i32) -> MockDiceRoller {
        if result >= 0 {
            MockDiceRoller::for_two_rolls(result as u32, 0)
        } else {
            MockDiceRoller::for_two_rolls(0, result.abs() as u32)
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

// RngDiceRoller

pub struct RngDiceRoller {}

impl RngDiceRoller {
    pub fn new() -> RngDiceRoller {
        RngDiceRoller {}
    }
}

impl DiceRoller for RngDiceRoller {
    fn roll(&self, sides: u32) -> u32 {
        use rand::Rng;
        rand::thread_rng().gen_range(1, sides + 1)
    }
}
