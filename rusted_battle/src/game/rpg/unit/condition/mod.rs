pub mod modifier;
pub mod set;
pub mod skill_modifier;

use crate::game::rpg::unit::skill::Skill;

pub trait Condition {
    fn get_id(&self) -> &str;

    fn get_skill_modifier(&self, _skill: &Skill) -> i32 {
        0
    }
}
