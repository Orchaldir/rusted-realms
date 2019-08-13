use std::cell::Cell;

#[derive(Debug)]
pub enum DiceRoller {
    Mocked {
        results: Vec<u32>,
        index: Cell<usize>,
    },
    Random,
}

impl DiceRoller {
    pub fn mock(results: Vec<u32>) -> DiceRoller {
        DiceRoller::Mocked {
            results,
            index: Cell::new(0),
        }
    }

    pub fn mock_two_rolls(positive_die: u32, negative_die: u32) -> DiceRoller {
        DiceRoller::mock(vec![positive_die, negative_die])
    }

    pub fn mock_die_minus_die(result: i32) -> DiceRoller {
        if result >= 0 {
            DiceRoller::mock_two_rolls(result as u32, 0)
        } else {
            DiceRoller::mock_two_rolls(0, result.abs() as u32)
        }
    }

    pub fn roll(&self, sides: u32) -> u32 {
        match self {
            DiceRoller::Mocked { results, index } => {
                let current_index = index.get();

                if current_index < results.len() {
                    let result = results[current_index];
                    index.set(current_index + 1);
                    return result;
                }

                panic!("Index {} is outside the result vector!", current_index);
            }
            DiceRoller::Random => {
                use rand::Rng;
                rand::thread_rng().gen_range(1, sides + 1)
            }
        }
    }

    pub fn roll_die_minus_die(&self, sides: u32) -> i32 {
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
