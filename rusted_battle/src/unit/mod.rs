use std::collections::HashMap;

pub mod skill;

#[derive(Debug)]
pub struct Unit<'a> {
    name: &'a str,
    skills: HashMap<&'a skill::Skill<'a>, i32>,
}

impl<'a> Unit<'a> {
    pub fn new(name: &'a str) -> Unit<'a> {
        Unit {
            name,
            skills: HashMap::new(),
        }
    }

    pub fn set_skill_rank(&mut self, skill: &'a skill::Skill<'a>, rank: i32) {
        self.skills.insert(skill, rank);
    }

    pub fn get_skill_rank(&self, skill: &'a skill::Skill<'a>) -> i32 {
        *self.skills.get(skill).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::skill::Skill;

    #[test]
    fn test_get_not_existing_skill() {
        let skill_a = Skill::new("a");
        let unit = Unit::new("test");

        assert_eq!(unit.get_skill_rank(&skill_a), 0)
    }

    #[test]
    fn test_skills() {
        let skill_a = Skill::new("a");
        let skill_b = Skill::new("b");
        let mut unit = Unit::new("test");
        unit.set_skill_rank(&skill_a, 2);
        unit.set_skill_rank(&skill_b, 6);

        assert_eq!(unit.get_skill_rank(&skill_a), 2);
        assert_eq!(unit.get_skill_rank(&skill_b), 6);
    }

    #[test]
    fn test_get_overwrite_skill() {
        let skill_a = Skill::new("a");
        let mut unit = Unit::new("test");

        unit.set_skill_rank(&skill_a, 2);
        unit.set_skill_rank(&skill_a, 5);

        assert_eq!(unit.get_skill_rank(&skill_a), 5)
    }
}
