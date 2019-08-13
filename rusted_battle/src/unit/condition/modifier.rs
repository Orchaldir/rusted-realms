use crate::unit::condition::Condition;
use crate::unit::skill::Skill;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Modifier {
    modifier: i32,
}

impl Modifier {
    pub fn new(modifier: i32) -> Modifier {
        Modifier { modifier }
    }
}

impl Condition for Modifier {
    fn get_skill_modifier(&self, skill: &Skill) -> i32 {
        self.modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier() {
        let skill_a = Skill::new("a");
        let condition = Modifier::new(3);

        assert_eq!(condition.get_skill_modifier(&skill_a), 3);
    }
}
