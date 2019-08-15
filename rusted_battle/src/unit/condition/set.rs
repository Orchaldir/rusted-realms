use super::*;
use std::collections::HashMap;

pub struct ConditionSet<'a> {
    conditions: HashMap<&'a str, &'a dyn Condition>,
}

impl<'a> ConditionSet<'a> {
    pub fn new() -> ConditionSet<'a> {
        ConditionSet {
            conditions: HashMap::new(),
        }
    }

    pub fn add(&mut self, condition: &'a dyn Condition) {
        self.conditions.insert(condition.get_id(), condition);
    }

    pub fn remove(&mut self, condition: &'a dyn Condition) {
        self.conditions.remove(condition.get_id());
    }

    pub fn get_skill_modifier(&self, skill: &Skill) -> i32 {
        self.conditions
            .values()
            .map(|c| c.get_skill_modifier(&skill))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::condition::modifier::Modifier;
    use crate::unit::condition::skill_modifier::SkillModifier;

    #[test]
    fn test_no_condition() {
        let skill_a = Skill::new("a");
        let skill_set: ConditionSet = ConditionSet::new();

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 0)
    }

    #[test]
    fn test_one_condition() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 2);

        let mut skill_set = ConditionSet::new();
        skill_set.add(&condition_a);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 2);
    }

    #[test]
    fn test_twice() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 2);

        let mut skill_set = ConditionSet::new();
        skill_set.add(&condition_a);
        skill_set.add(&condition_a);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 2);
    }

    #[test]
    fn test_two_conditions_with_different_skills() {
        let skill_a = Skill::new("a");
        let skill_b = Skill::new("b");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 3);
        let condition_b = SkillModifier::new("cond_b", &skill_b, -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 3);
        assert_eq!(skill_set.get_skill_modifier(&skill_b), -4);
    }

    #[test]
    fn test_two_conditions_with_same_skills() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 3);
        let condition_b = SkillModifier::new("cond_b", &skill_a, -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), -1);
    }
    #[test]
    fn test_different_condition_types() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 3);
        let condition_b = Modifier::new("cond_b", -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), -1);
    }

    #[test]
    fn test_remove() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new("cond_a", &skill_a, 3);
        let condition_b = SkillModifier::new("cond_b", &skill_a, -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);
        skill_set.remove(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 3);
    }
}
