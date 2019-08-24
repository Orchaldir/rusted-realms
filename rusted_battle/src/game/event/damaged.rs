use crate::game::rpg::unit::health::damage::Damage;

pub struct Damaged<'a> {
    pub target: u32,
    pub attacker: u32,
    pub damage: &'a Damage,
}
