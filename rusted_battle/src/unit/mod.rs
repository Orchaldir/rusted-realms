use crate::unit::skill::set::SkillSet;

pub mod health;
pub mod skill;

#[derive(Debug)]
pub struct Unit<'a> {
    name: &'a str,
    skill_set: SkillSet<'a>,
}

impl<'a> Unit<'a> {
    pub fn new(name: &'a str) -> Unit<'a> {
        Unit {
            name,
            skill_set: SkillSet::new(),
        }
    }
}
