extern crate rand;

#[derive(Debug)]
pub struct Checker {
    sides: u32
}

impl Checker {
    pub fn check(self, value: i32, difficulty: i32) -> i32 {
        let positive_random = roll_dice(self.sides);
        let negative_random = roll_dice(self.sides);
        let random_modifier = positive_random - negative_random;
        println!("Roll d{0}-d{0}={1}-{2}={3}", self.sides, positive_random, negative_random, random_modifier);
        value - difficulty + random_modifier
    }
}

pub fn build_checker(sides: u32) -> Checker {
    Checker { sides }
}

fn roll_dice(sides: u32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(1, sides as i32 + 1)
}