#[derive(Debug, PartialEq)]
pub enum DamageType {
    Fire,
    Magic,
    Normal,
}

#[derive(Debug)]
pub struct Damage {
    pub rank: i32,
    pub damage_type: DamageType,
}

impl Damage {
    pub fn new(rank: i32) -> Damage {
        Damage {
            rank,
            damage_type: DamageType::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_damage() {
        let damage = Damage::new(42);

        assert_eq!(damage.rank, 42);
        assert_eq!(damage.damage_type, DamageType::Normal);
    }
}
