use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SkillSet<'a> {
    skills: HashMap<&'a Skill<'a>, i32>,
}

impl<'a> SkillSet<'a> {
    pub fn new() -> SkillSet<'a> {
        SkillSet {
            skills: HashMap::new(),
        }
    }

    pub fn set_skill_rank(&mut self, skill: &'a Skill<'a>, rank: i32) {
        self.skills.insert(skill, rank);
    }

    pub fn get_skill_rank(&self, skill: &'a Skill<'a>) -> i32 {
        *self.skills.get(skill).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_not_existing_skill() {
        let skill_a = Skill::new("a");
        let skill_set = SkillSet::new();

        assert_eq!(skill_set.get_skill_rank(&skill_a), 0)
    }

    #[test]
    fn test_skills() {
        let skill_a = Skill::new("a");
        let skill_b = Skill::new("b");
        let mut skill_set = SkillSet::new();
        skill_set.set_skill_rank(&skill_a, 2);
        skill_set.set_skill_rank(&skill_b, 6);

        assert_eq!(skill_set.get_skill_rank(&skill_a), 2);
        assert_eq!(skill_set.get_skill_rank(&skill_b), 6);
    }

    #[test]
    fn test_get_overwrite_skill() {
        let skill_a = Skill::new("a");
        let mut skill_set = SkillSet::new();

        skill_set.set_skill_rank(&skill_a, 2);
        skill_set.set_skill_rank(&skill_a, 5);

        assert_eq!(skill_set.get_skill_rank(&skill_a), 5)
    }
}
