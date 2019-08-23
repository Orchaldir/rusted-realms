use crate::game::component::health::HealthComponent;
use crate::game::component::statistics::StatisticsComponent;
use crate::game::rpg::unit::health::damage::Damage;
use crate::game::rpg::unit::skill::check::{CheckResult, Checker};
use crate::game::rpg::unit::skill::Skill;
use crate::utils::ecs::component::ComponentManager;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HealthSystem<'a> {
    health_components: Rc<RefCell<ComponentManager<HealthComponent>>>,
    statistics_components: Rc<RefCell<ComponentManager<StatisticsComponent<'a>>>>,
    checker: &'a Checker<'a>,
    toughness: &'a Skill<'a>,
}

impl<'a> HealthSystem<'a> {
    pub fn new(
        health_components: Rc<RefCell<ComponentManager<HealthComponent>>>,
        statistics_components: Rc<RefCell<ComponentManager<StatisticsComponent<'a>>>>,
        checker: &'a Checker<'a>,
        toughness: &'a Skill<'a>,
    ) -> HealthSystem<'a> {
        HealthSystem {
            health_components,
            statistics_components,
            checker,
            toughness,
        }
    }

    pub fn take_damage(&mut self, entity: u32, damage: &Damage) {
        let h: &mut ComponentManager<HealthComponent> = &mut self.health_components.borrow_mut();
        let mut health = h.get_mut(entity).unwrap();
        let s: &ComponentManager<StatisticsComponent> = &self.statistics_components.borrow();
        let statistics = s.get(entity).unwrap();

        let rank = statistics.get_skill_rank(self.toughness) - health.penalty as i32;

        match self.checker.check(rank, damage.rank) {
            CheckResult::Success { degree } if degree > 1 => {}
            CheckResult::Success { degree: _ } => {
                health.penalty += 1;
            }
            CheckResult::Failure { degree: _ } => {
                health.is_alive = false;
            }
        }
    }
}
