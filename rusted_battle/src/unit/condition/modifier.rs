use crate::unit::condition::Condition;
use crate::unit::skill::Skill;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Modifier<'a> {
    id: &'a str,
    modifier: i32,
}

impl<'a> Modifier<'a> {
    pub fn new(id: &'a str, modifier: i32) -> Modifier<'a> {
        Modifier { id, modifier }
    }
}

impl<'a> Condition for Modifier<'a> {
    fn get_id(&self) -> &str {
        self.id
    }

    fn get_skill_modifier(&self, _skill: &Skill) -> i32 {
        self.modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let skill_a = Skill::new("a");
        let condition = Modifier::new("test", 3);

        assert_eq!(condition.get_id(), "test");
        assert_eq!(condition.get_skill_modifier(&skill_a), 3);
    }
}
