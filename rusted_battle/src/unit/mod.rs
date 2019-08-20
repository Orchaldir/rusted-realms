use crate::unit::condition::set::ConditionSet;
use crate::unit::skill::set::SkillSet;
use crate::unit::skill::Skill;

pub mod condition;
pub mod health;
pub mod skill;

pub struct Unit<'a> {
    name: &'a str,
    skill_set: SkillSet<'a>,
    conditions: ConditionSet<'a>,
}

impl<'a> Unit<'a> {
    pub fn new(name: &'a str) -> Unit<'a> {
        Unit {
            name,
            skill_set: SkillSet::new(),
            conditions: ConditionSet::new(),
        }
    }

    pub fn get_skill_rank(&self, skill: &'a Skill<'a>) -> i32 {
        let rank = self.skill_set.get_skill_rank(skill);
        let modifier = self.conditions.get_skill_modifier(skill);

        rank + modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::condition::skill_modifier::SkillModifier;

    #[test]
    fn test_get_skill_rank() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 2);
        let mut unit = Unit::new("test");

        unit.conditions.add(&condition_a);
        unit.skill_set.set_skill_rank(&skill_a, 3);

        assert_eq!(unit.get_skill_rank(&skill_a), 5)
    }
}
