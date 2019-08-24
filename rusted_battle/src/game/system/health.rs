use crate::game::component::health::HealthComponent;
use crate::game::component::statistics::StatisticsComponent;
use crate::game::event::damaged::Damaged;
use crate::game::event::killed::Killed;
use crate::game::rpg::unit::health::damage::Damage;
use crate::game::rpg::unit::skill::check::{CheckResult, Checker};
use crate::game::rpg::unit::skill::Skill;
use crate::utils::ecs::component::ComponentManager;
use crate::utils::ecs::event::EventListener;
use crate::utils::ecs::event::EventPublisher;
use std::cell::RefCell;
use std::rc::Rc;

pub struct HealthSystem<'a> {
    health_components: Rc<RefCell<ComponentManager<HealthComponent>>>,
    statistics_components: Rc<RefCell<ComponentManager<StatisticsComponent<'a>>>>,
    killed_publisher: &'a EventPublisher<Killed>,
    checker: &'a Checker<'a>,
    toughness: &'a Skill<'a>,
}

impl<'a> HealthSystem<'a> {
    pub fn new(
        health_components: Rc<RefCell<ComponentManager<HealthComponent>>>,
        statistics_components: Rc<RefCell<ComponentManager<StatisticsComponent<'a>>>>,
        killed_publisher: &'a EventPublisher<Killed>,
        checker: &'a Checker<'a>,
        toughness: &'a Skill<'a>,
    ) -> HealthSystem<'a> {
        HealthSystem {
            health_components,
            statistics_components,
            killed_publisher,
            checker,
            toughness,
        }
    }

    pub fn take_damage(&mut self, target: u32, attacker: u32, damage: &Damage) {
        let h: &mut ComponentManager<HealthComponent> = &mut self.health_components.borrow_mut();
        let mut health = h.get_mut(target).unwrap();
        let s: &ComponentManager<StatisticsComponent> = &self.statistics_components.borrow();
        let statistics = s.get(target).unwrap();

        let rank = statistics.get_skill_rank(self.toughness) - health.penalty as i32;

        match self.checker.check(rank, damage.rank) {
            CheckResult::Success { degree } if degree > 1 => {}
            CheckResult::Success { degree: _ } => {
                health.penalty += 1;
            }
            CheckResult::Failure { degree: _ } => {
                health.is_alive = false;
                let killed = Killed {
                    target,
                    killer: attacker,
                };
                self.killed_publisher.publish(&killed);
            }
        }
    }
}

impl<'a> EventListener<Damaged<'a>> for HealthSystem<'a> {
    fn update(&mut self, event: &Damaged) {
        self.take_damage(event.target, event.attacker, event.damage);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::component::health::HealthComponent;
    use crate::game::component::statistics::StatisticsComponent;
    use crate::game::event::damaged::Damaged;
    use crate::game::event::killed::Killed;
    use crate::game::rpg::unit::health::damage::Damage;
    use crate::game::rpg::unit::skill::check::build_checker;
    use crate::game::rpg::unit::skill::check::{CheckResult, Checker};
    use crate::game::rpg::unit::skill::Skill;
    use crate::utils::dice_roller::DiceRoller;
    use crate::utils::ecs::component::ComponentManager;
    use crate::utils::ecs::event::EventListener;
    use crate::utils::ecs::event::EventPublisher;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_killed() {
        let id: u32 = 0;

        let health_manager = Rc::new(RefCell::new(ComponentManager::new()));
        health_manager.borrow_mut().add(id, HealthComponent::new());

        let statistics_manager = Rc::new(RefCell::new(ComponentManager::new()));
        statistics_manager
            .borrow_mut()
            .add(id, StatisticsComponent::new());

        let damage = Damage::new(100);
        let damaged = Damaged {
            target: id,
            attacker: 99,
            damage: &damage,
        };

        let mut killed_publisher = EventPublisher::<Killed>::new();

        let roller = DiceRoller::mock_die_minus_die(0);
        let checker = build_checker(6, 3, &roller);

        let toughness = Skill::new("Toughness");

        let mut health_system = HealthSystem {
            health_components: health_manager.clone(),
            statistics_components: statistics_manager.clone(),
            killed_publisher: &killed_publisher,
            checker: &checker,
            toughness: &toughness,
        };

        health_system.update(&damaged);

        assert_health(id, health_manager.clone(), false, 0);
    }

    fn assert_health(
        id: u32,
        health_manager: Rc<RefCell<ComponentManager<HealthComponent>>>,
        is_alive: bool,
        penalty: u32,
    ) {
        let ref_mut = health_manager.borrow_mut();
        let health = ref_mut.get(id).unwrap();
        assert_eq!(health.is_alive, is_alive);
        assert_eq!(health.penalty, penalty);
    }
}
