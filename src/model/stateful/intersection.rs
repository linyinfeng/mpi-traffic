use crate::model::{
    common::{Around, TurnRule},
    stateless,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Intersection {
    Crossroad {
        current: Around<TurnRule>,
        switch_state: SwitchState,
    },
    TJunction {
        current: Around<TurnRule>,
        switch_state: SwitchState,
    },
    Turn,
    Straight,
    End,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SwitchState {
    LoopTimeout {
        remain_time: f64,
        time_index: usize,
        rule_index: usize,
    },
}

impl Intersection {
    pub fn update_current(&mut self, stateless: &stateless::Intersection) {
        match (self, stateless) {
            (
                Intersection::Crossroad {
                    current,
                    switch_state:
                        SwitchState::LoopTimeout {
                            rule_index,
                            ..
                        },
                },
                stateless::Intersection::Crossroad {
                    rules,
                    ..
                },
            ) => {
                *current = rules[*rule_index];
            },
            (
                Intersection::TJunction {
                    current,
                    switch_state:
                        SwitchState::LoopTimeout {
                            rule_index,
                            ..
                        },
                },
                stateless::Intersection::TJunction {
                    single,
                    rule_set,
                    ..
                },
            ) => {
                let rule = &rule_set[*rule_index];
                *current.get_mut(*single) = rule.for_single;
                let driver_direction = single.turn_back();
                *current.get_mut(driver_direction.turn_left()) = rule.for_left;
                *current.get_mut(driver_direction.turn_right()) = rule.for_right;
                *current.get_mut(single.turn_back()) = TurnRule::empty();
            },
            (Intersection::Crossroad { .. }, _) => unreachable!(),
            (Intersection::TJunction { .. }, _) => unreachable!(),
            _ => (), // no need to update current
        }
    }
}
