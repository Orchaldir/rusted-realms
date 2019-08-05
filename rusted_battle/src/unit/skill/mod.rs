pub mod check;

use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Skill<'a> {
    name: &'a str,
}

impl<'a> Skill<'a> {
    pub fn new(name: &'a str) -> Skill<'a> {
        Skill { name }
    }
}

#[derive(Debug)]
pub struct SkillManager<'a> {
    skills: HashMap<&'a str, Skill<'a>>,
}

impl<'a> SkillManager<'a> {
    pub fn new() -> SkillManager<'a> {
        SkillManager {
            skills: HashMap::new(),
        }
    }

    pub fn create(&mut self, name: &'a str) {
        if self.skills.contains_key(name) {
            panic!("Skill '{}' already exists!", name)
        }

        let skill = Skill { name };
        self.skills.insert(name, skill);
    }

    pub fn get(&self, name: &'a str) -> &Skill<'a> {
        if !self.skills.contains_key(name) {
            panic!("Skill '{}' does not exist!", name)
        }
        self.skills.get(name).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_skills() {
        let mut skill_manager = SkillManager::new();

        skill_manager.create("a");
        skill_manager.create("b");

        let skill_a = skill_manager.get("a");
        let skill_b = skill_manager.get("b");

        assert_eq!(skill_a.name, "a");
        assert_eq!(skill_b.name, "b");
    }

    #[test]
    #[should_panic(expected = "Skill 'a' already exists!")]
    fn test_create_skill_twice() {
        let mut skill_manager = SkillManager::new();

        skill_manager.create("a");
        skill_manager.create("a");
    }

    #[test]
    #[should_panic(expected = "Skill 'c' does not exist!")]
    fn test_get_not_existing_skill() {
        let mut skill_manager = SkillManager::new();

        skill_manager.get("c");
    }
}
