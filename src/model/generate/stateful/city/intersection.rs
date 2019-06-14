use crate::model::{
    stateful::{intersection, Intersection},
    stateless,
};

pub fn generate_intersection_from_stateless(
    stateless_model: &stateless::Intersection,
) -> Intersection {
    let mut result = match stateless_model {
        stateless::Intersection::Crossroad {
            switch_rule: stateless::intersection::SwitchRule::LoopTimeout { times },
            ..
        } => Intersection::Crossroad {
            current: Default::default(),
            switch_state: intersection::SwitchState::LoopTimeout {
                remain_time: times[0],
                time_index: 0,
                rule_index: 0,
            },
        },
        stateless::Intersection::TJunction {
            switch_rule: stateless::intersection::SwitchRule::LoopTimeout { times },
            ..
        } => Intersection::TJunction {
            current: Default::default(),
            switch_state: intersection::SwitchState::LoopTimeout {
                remain_time: times[0],
                time_index: 0,
                rule_index: 0,
            },
        },
        stateless::Intersection::Turn { .. } => Intersection::Turn,
        stateless::Intersection::Straight => Intersection::Straight,
        stateless::Intersection::End { .. } => Intersection::End,
    };
    result.update_current(stateless_model);
    result
}
