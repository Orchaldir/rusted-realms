extern crate rand;

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

    fn attack(self, target: Character) -> AttackResult {
        let diff = roll(self.fighting, target.fighting);

        if diff > 0 {
            let damage = diff + self.strength;
            AttackResult::Hit {damage }
        } else {
            AttackResult::Miss
        }
    }
}

fn roll(value: i32, difficulty: i32) -> i32 {
    let sides = 10;
    let positive_random = roll_dice(sides);
    let negative_random = roll_dice(sides);
    let random_modifier = positive_random - negative_random;
    println!("Roll d{0}-d{0}={1}-{2}={3}", sides, positive_random, negative_random, random_modifier);
    value - difficulty + random_modifier
}

fn roll_dice(sides: i32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(1, sides + 1)
}


fn main() {
    let character0 = Character { fighting: 6, strength: 3 };
    let character1 = Character { fighting: 5, strength: 8 };

    let result = character0.attack(character1);

    match result {
        AttackResult::Hit {damage} => println!("A hit with {} damage!", damage),
        AttackResult::Miss => println!("A miss :(")
    }
}
