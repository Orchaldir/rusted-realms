extern crate rand;

#[derive(Debug)]
pub struct Checker<'a, T: rand::Rng> {
    die_sides: u32,
    size_of_degree: u32,
    rng: &'a mut T,
}

#[derive(Debug)]
pub enum CheckResult {
    Success { degree: u32 },
    Failure { degree: u32 },
}

impl<'a, T> Checker<'a, T>
where
    T: rand::Rng,
{
    pub fn check(&mut self, skill: i32, difficulty: i32) -> CheckResult {
        let positive_die = self.roll_die();
        let negative_die = self.roll_die();
        let random_modifier = positive_die - negative_die;
        println!(
            "Roll d{0}-d{0}={1}-{2}={3}",
            self.die_sides, positive_die, negative_die, random_modifier
        );

        let diff = skill - difficulty + random_modifier;
        let degree = 1 + diff.abs() as u32 / self.size_of_degree;
        println!("diff={} degree={}", diff, degree);

        if diff >= 0 {
            CheckResult::Success { degree }
        } else {
            CheckResult::Failure { degree }
        }
    }

    fn roll_die(&mut self) -> i32 {
        self.rng.gen_range(1, self.die_sides as i32 + 1)
    }
}

pub fn build_checker<T>(die_sides: u32, size_of_degree: u32, rng: &mut T) -> Checker<T>
where
    T: rand::Rng,
{
    Checker {
        die_sides,
        size_of_degree,
        rng,
    }
}
