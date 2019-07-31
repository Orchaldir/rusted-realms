extern crate rand;

use crate::utils::dice_roller::*;

#[derive(Debug)]
pub struct Checker<'a, T: DiceRoller> {
    die_sides: u32,
    size_of_degree: u32,
    dice_roller: &'a T,
}

#[derive(Debug)]
pub enum CheckResult {
    Success { degree: u32 },
    Failure { degree: u32 },
}

impl<'a, T> Checker<'a, T>
where
    T: DiceRoller,
{
    pub fn check(&self, skill: i32, difficulty: i32) -> CheckResult {
        let random_modifier = self.dice_roller.roll_die_minus_die(self.die_sides);
        let diff = skill - difficulty + random_modifier;
        let degree = 1 + diff.abs() as u32 / self.size_of_degree;

        println!("diff={} degree={}", diff, degree);

        if diff >= 0 {
            CheckResult::Success { degree }
        } else {
            CheckResult::Failure { degree }
        }
    }
}

pub fn build_checker<T>(die_sides: u32, size_of_degree: u32, dice_roller: &T) -> Checker<T>
where
    T: DiceRoller,
{
    Checker {
        die_sides,
        size_of_degree,
        dice_roller,
    }
}
