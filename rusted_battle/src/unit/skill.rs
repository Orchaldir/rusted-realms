extern crate rand;

#[derive(Debug)]
pub struct Checker {
    die_sides: u32,
    size_of_degree: u32
}

#[derive(Debug)]
pub enum CheckResult {
    Success { degree: u32 },
    Failure { degree: u32 }
}

impl Checker {
    pub fn check(self, skill: i32, difficulty: i32) -> CheckResult {
        let positive_die = roll_die(self.die_sides);
        let negative_die = roll_die(self.die_sides);
        let random_modifier = positive_die - negative_die;
        println!("Roll d{0}-d{0}={1}-{2}={3}", self.die_sides, positive_die, negative_die, random_modifier);
        let diff = skill - difficulty + random_modifier;
        let degree = 1 + diff.abs() as u32 / self.size_of_degree;
        println!("diff={} degree={}",diff, degree);

        if diff >= 0 {
            CheckResult::Success {degree}
        }
        else {
            CheckResult::Failure {degree}
        }
    }
}

pub fn build_checker(die_sides: u32, size_of_degree: u32) -> Checker {
    Checker { die_sides, size_of_degree }
}

fn roll_die(sides: u32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(1, sides as i32 + 1)
}