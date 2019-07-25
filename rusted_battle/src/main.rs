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
        let diff = self.fighting - target.fighting;

        if diff > 0 {
            let damage = diff + self.strength;
            AttackResult::Hit {damage }
        } else {
            AttackResult::Miss
        }
    }
}

fn main() {
    let character0 = Character { fighting: 6, strength: 3 };
    let character1 = Character { fighting: 5, strength: 8 };

    let result = character0.attack(character1);

    println!("Result: {:?}", result);

    match result {
        AttackResult::Hit {damage} => println!("A hit with {} damage!", damage),
        AttackResult::Miss => println!("A miss :(")
    }
}
