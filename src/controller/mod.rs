use crate::model::{stateful, stateless};
use piston_window::{Input, UpdateArgs};
use log::trace;

pub mod car_map;

#[derive(Clone, Debug)]
pub struct Controller {
    elapsed_time: f64,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            elapsed_time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        stateful: &mut stateful::Model,
        stateless: &stateless::Model,
        args: UpdateArgs,
    ) {
        self.elapsed_time += args.dt;
        trace!("elapsed time: {:.5}, dt: {:.5}", self.elapsed_time, args.dt);

        for (stateful_intersection, stateless_intersection) in stateful
            .city
            .board
            .intersections
            .iter_mut()
            .zip(stateless.city.board.intersections.iter())
        {
            if let Some(stateful_intersection) = stateful_intersection.as_mut() {
                let stateless_intersection = stateless_intersection.as_ref().unwrap();
                self.update_intersection(
                    stateful_intersection,
                    stateless_intersection,
                    args,
                );
                stateful_intersection.update_current(stateless_intersection);
            }
        }
    }

    pub fn input(
        &mut self,
        _stateful: &mut stateful::Model,
        _stateless: &stateless::Model,
        _input: Input,
    ) {
    }

    fn update_intersection(
        &self,
        stateful: &mut stateful::Intersection,
        stateless: &stateless::Intersection,
        UpdateArgs { dt }: UpdateArgs,
    ) {
        match (stateful, stateless) {
            (
                stateful::Intersection::Crossroad {
                    switch_state:
                        stateful::intersection::SwitchState::LoopTimeout {
                            remain_time,
                            time_index,
                            rule_index,
                        },
                    ..
                },
                stateless::Intersection::Crossroad {
                    rules,
                    switch_rule: stateless::intersection::SwitchRule::LoopTimeout { times },
                    ..
                },
            ) => {
                *remain_time -= dt;
                if *remain_time < 0.0 {
                    // Timeout
                    *time_index += 1;
                    *time_index %= times.len();
                    *rule_index += 1;
                    *rule_index %= rules.len();
                    *remain_time += times[*time_index]; // Set new timeout
                }
            },
            (
                stateful::Intersection::TJunction {
                    switch_state:
                        stateful::intersection::SwitchState::LoopTimeout {
                            remain_time,
                            time_index,
                            rule_index,
                        },
                    ..
                },
                stateless::Intersection::TJunction {
                    rule_set,
                    switch_rule: stateless::intersection::SwitchRule::LoopTimeout { times },
                    ..
                },
            ) => {
                *remain_time -= dt;
                if *remain_time < 0.0 {
                    // Timeout
                    *time_index += 1;
                    *time_index %= times.len();
                    *rule_index += 1;
                    *rule_index %= rule_set.len();
                    *remain_time += times[*time_index]; // Set new timeout
                }
            },
            (stateful::Intersection::Crossroad { .. }, _) => unreachable!(),
            (stateful::Intersection::TJunction { .. }, _) => unreachable!(),
            _ => (), // no need to update current
        }
    }
}
