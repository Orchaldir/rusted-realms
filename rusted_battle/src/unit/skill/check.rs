extern crate rand;

use crate::utils::dice_roller::*;

#[derive(Debug)]
pub struct Checker<'a> {
    die_sides: u32,
    size_of_degree: u32,
    dice_roller: &'a DiceRoller,
}

#[derive(Debug)]
pub enum CheckResult {
    Success { degree: u32 },
    Failure { degree: u32 },
}

impl<'a> Checker<'a> {
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

pub fn build_checker(die_sides: u32, size_of_degree: u32, dice_roller: &DiceRoller) -> Checker {
    Checker {
        die_sides,
        size_of_degree,
        dice_roller,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::dice_roller::DiceRoller;

    fn assert_success(roll_result: i32, skill: i32, difficulty: i32, expected: u32) {
        let roller = DiceRoller::mock_die_minus_die(roll_result);
        let checker = build_checker(6, 3, &roller);

        match checker.check(skill, difficulty) {
            CheckResult::Success { degree } => {
                assert_eq!(degree, expected, "Check degree of success")
            }
            CheckResult::Failure { .. } => panic!("Result should be a success!"),
        }
    }

    fn assert_failure(roll_result: i32, skill: i32, difficulty: i32, expected: u32) {
        let roller = DiceRoller::mock_die_minus_die(roll_result);
        let checker = build_checker(6, 3, &roller);

        match checker.check(skill, difficulty) {
            CheckResult::Success { .. } => panic!("Result should be a failure!"),
            CheckResult::Failure { degree } => {
                assert_eq!(degree, expected, "Check degree of failure")
            }
        }
    }

    #[test]
    fn test_draw_is_one_success() {
        assert_success(0, 0, 0, 1);
    }

    #[test]
    fn test_roll_result() {
        assert_failure(-6, 0, 0, 3);
        assert_failure(-5, 0, 0, 2);
        assert_failure(-4, 0, 0, 2);
        assert_failure(-3, 0, 0, 2);
        assert_failure(-2, 0, 0, 1);
        assert_failure(-1, 0, 0, 1);
        assert_success(1, 0, 0, 1);
        assert_success(2, 0, 0, 1);
        assert_success(3, 0, 0, 2);
        assert_success(4, 0, 0, 2);
        assert_success(5, 0, 0, 2);
        assert_success(6, 0, 0, 3);
    }

    #[test]
    fn test_skill() {
        assert_success(0, 1, 0, 1);
        assert_success(0, 2, 0, 1);
        assert_success(0, 3, 0, 2);
        assert_success(0, 4, 0, 2);
        assert_success(0, 5, 0, 2);
        assert_success(0, 6, 0, 3);
    }

    #[test]
    fn test_difficulty() {
        assert_failure(0, 0, 6, 3);
        assert_failure(0, 0, 5, 2);
        assert_failure(0, 0, 4, 2);
        assert_failure(0, 0, 3, 2);
        assert_failure(0, 0, 2, 1);
        assert_failure(0, 0, 1, 1);
    }

    #[test]
    fn test_combinations() {
        assert_failure(-2, 0, 4, 3);
        assert_failure(-3, 1, 0, 1);
        assert_success(2, 4, 6, 1);
        assert_success(5, 5, 7, 2);
    }
}
