#[derive(Debug)]
struct Character {
    fighting: i32
}

#[derive(Debug)]
enum AttackResult {
    Hit(i32),
    Miss,
}

impl Character {

    fn attack(self, target: Character) -> AttackResult {
        let margin_of_success = self.fighting - target.fighting;

        if margin_of_success > 0 {
            AttackResult::Hit(margin_of_success)
        } else {
            AttackResult::Miss
        }
    }
}

fn main() {
    let character0 = Character { fighting: 6};
    let character1 = Character { fighting: 5};

    let result = character0.attack(character1);

    println!("Result: {:?}", result);
}
