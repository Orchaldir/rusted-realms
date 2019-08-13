use crate::unit::condition::Condition;
use crate::unit::skill::Skill;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct SkillModifier<'a> {
    skill: &'a Skill<'a>,
    modifier: i32,
}

impl<'a> SkillModifier<'a> {
    pub fn new(skill: &'a Skill<'a>, modifier: i32) -> SkillModifier<'a> {
        SkillModifier { skill, modifier }
    }
}

impl<'a> Condition for SkillModifier<'a> {
    fn get_skill_modifier(&self, skill: &Skill) -> i32 {
        if skill == self.skill {
            self.modifier
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_modifier() {
        let skill_a = Skill::new("a");
        let skill_b = Skill::new("b");
        let condition = SkillModifier::new(&skill_a, 3);

        assert_eq!(condition.get_skill_modifier(&skill_a), 3);
        assert_eq!(condition.get_skill_modifier(&skill_b), 0);
    }
}
