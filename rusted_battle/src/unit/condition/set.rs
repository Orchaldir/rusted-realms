use super::*;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub struct ConditionSet<'a, T>
where
    T: Condition + Eq + Hash,
{
    conditions: HashSet<&'a T>,
}

impl<'a, T> ConditionSet<'a, T>
where
    T: Condition + Eq + Hash,
{
    pub fn new() -> ConditionSet<'a, T> {
        ConditionSet {
            conditions: HashSet::new(),
        }
    }

    pub fn add(&mut self, condition: &'a T) {
        self.conditions.insert(condition);
    }

    pub fn remove(&mut self, condition: &'a T) {
        self.conditions.remove(condition);
    }

    pub fn get_skill_modifier(&self, skill: &Skill) -> i32 {
        self.conditions
            .iter()
            .map(|c| c.get_skill_modifier(&skill))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_condition() {
        let skill_a = Skill::new("a");
        let skill_set: ConditionSet<SkillModifier> = ConditionSet::new();

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 0)
    }

    #[test]
    fn test_one_condition() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new(&skill_a, 2);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 2);
    }

    #[test]
    fn test_two_conditions_with_different_skills() {
        let skill_a = Skill::new("a");
        let skill_b = Skill::new("b");
        let condition_a = SkillModifier::new(&skill_a, 3);
        let condition_b = SkillModifier::new(&skill_b, -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 3);
        assert_eq!(skill_set.get_skill_modifier(&skill_b), -4);
    }

    #[test]
    fn test_two_conditions_with_same_skills() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new(&skill_a, 3);
        let condition_b = SkillModifier::new(&skill_a, -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), -1);
    }

    #[test]
    fn test_remove() {
        let skill_a = Skill::new("a");
        let condition_a = SkillModifier::new(&skill_a, 3);
        let condition_b = SkillModifier::new(&skill_a, -4);
        let mut skill_set = ConditionSet::new();

        skill_set.add(&condition_a);
        skill_set.add(&condition_b);
        skill_set.remove(&condition_b);

        assert_eq!(skill_set.get_skill_modifier(&skill_a), 3);
    }
}
