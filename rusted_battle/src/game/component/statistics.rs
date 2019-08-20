use crate::unit::condition::set::ConditionSet;
use crate::unit::skill::set::SkillSet;
use crate::unit::skill::Skill;

pub struct StatisticsComponent<'a> {
    skills: SkillSet<'a>,
    conditions: ConditionSet<'a>,
}

impl<'a> StatisticsComponent<'a> {
    pub fn new() -> StatisticsComponent<'a> {
        StatisticsComponent {
            skills: SkillSet::new(),
            conditions: ConditionSet::new(),
        }
    }

    pub fn get_skill_rank(&self, skill: &'a Skill<'a>) -> i32 {
        let rank = self.skills.get_skill_rank(skill);
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
        let mut statistics = StatisticsComponent::new();

        statistics.conditions.add(&condition_a);
        statistics.skills.set_skill_rank(&skill_a, 3);

        assert_eq!(statistics.get_skill_rank(&skill_a), 5)
    }
}
