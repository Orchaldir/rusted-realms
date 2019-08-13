pub mod modifier;
pub mod set;
pub mod skill_modifier;

use crate::unit::skill::Skill;

pub trait Condition {
    fn get_skill_modifier(&self, skill: &Skill) -> i32 {
        0
    }
}
