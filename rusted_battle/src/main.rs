mod unit;

extern crate rand;

use unit::skill::*;

#[derive(Debug)]
struct Character {
    fighting: i32,
    strength: i32
}

#[derive(Debug)]
enum AttackResult {
    Hit {damage: i32},
    Miss,
}

impl Character {

    fn attack<'a, T: rand::Rng>(self, target: Character, mut checker: Checker<T>) -> AttackResult {
        match checker.check(self.fighting, target.fighting) {
            CheckResult::Success {degree} => {
                let damage = self.strength * degree as i32;
                AttackResult::Hit {damage }
            },
            _ => AttackResult::Miss
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let checker = unit::skill::build_checker(10, 5, &mut rng);
    let character0 = Character { fighting: 6, strength: 3 };
    let character1 = Character { fighting: 5, strength: 8 };

    let result = character0.attack(character1, checker);

    match result {
        AttackResult::Hit {damage} => println!("A hit with {} damage!", damage),
        AttackResult::Miss => println!("A miss :(")
    }
}
