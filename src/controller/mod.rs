use crate::{
    communication,
    info::Info,
    model::{stateful, stateless},
};
use mpi::{collective::CommunicatorCollectives, topology::Rank};
use piston_window::{Button, ButtonArgs, ButtonState, Input, Motion, MouseButton, UpdateArgs};
use structopt::StructOpt;

pub mod car_map;

#[derive(Clone, Debug)]
pub struct Controller {
    pub mouse_left_button_down: bool,
    pub mouse_left_button_down_location: Option<(f64, f64)>,
    pub start_drag_location: Option<(f64, f64)>,
    pub settings: ControllerSettings,
    pub update_controller: UpdateController,
}

#[derive(StructOpt, Clone, Debug)]
pub struct ControllerSettings {
    #[structopt(name = "zoom-step", long = "zoom-step", default_value = "0.1")]
    pub zoom_step: f64,
}

impl Controller {
    pub fn new(update_controller: UpdateController, settings: ControllerSettings) -> Self {
        Self {
            mouse_left_button_down: false,
            mouse_left_button_down_location: None,
            start_drag_location: None,
            settings,
            update_controller,
        }
    }
}

impl Controller {
    pub fn input(
        &mut self,
        info: &mut Info,
        _stateful: &mut stateful::Model,
        _stateless: &stateless::Model,
        input: Input,
    ) {
        match input {
            Input::Button(ButtonArgs {
                state,
                button: Button::Mouse(MouseButton::Left),
                ..
            }) => {
                match state {
                    ButtonState::Press => {
                        self.mouse_left_button_down = true;
                        self.start_drag_location = Some((info.x, info.y));
                    },
                    ButtonState::Release => {
                        self.mouse_left_button_down = false;
                        self.start_drag_location = None;
                    },
                };
                self.mouse_left_button_down_location = None;
            },
            Input::Move(Motion::MouseCursor([x, y])) => {
                if self.mouse_left_button_down {
                    if let Some((origin_x, origin_y)) = self.mouse_left_button_down_location {
                        if let Some((start_drag_x, start_drag_y)) = self.start_drag_location {
                            info.x = start_drag_x + x - origin_x;
                            info.y = start_drag_y + y - origin_y;
                        }
                    } else {
                        self.mouse_left_button_down_location = Some((x, y));
                    }
                }
            },
            Input::Move(Motion::MouseScroll([_x, y])) => {
                info.zoom += y * self.settings.zoom_step;
            },
            _ => (),
        }
    }
}

impl Controller {
    pub fn update<Comm>(
        &mut self,
        root: Rank,
        communicator: Comm,
        _info: &mut Info,
        stateful: &mut stateful::Model,
        stateless: &stateless::Model,
        args: UpdateArgs,
    ) where
        Comm: CommunicatorCollectives + Clone,
    {
        self.update_controller
            .update(root, communicator, stateful, stateless, args);
    }
}

#[derive(Clone, Debug)]
pub struct UpdateController;

impl UpdateController {
    pub fn new() -> Self {
        Self
    }

    pub fn update<Comm>(
        &mut self,
        root: Rank,
        communicator: Comm,
        stateful: &mut stateful::Model,
        stateless: &stateless::Model,
        args: UpdateArgs,
    ) where
        Comm: CommunicatorCollectives + Clone,
    {
        self.update_city(
            root,
            communicator.clone(),
            &mut stateful.city,
            &stateless.city,
            args,
        );
        self.update_cars(root, communicator, stateful, stateless, args);
    }

    pub fn update_cars<Comm>(
        &mut self,
        _root: Rank,
        _communicator: Comm,
        stateful: &mut stateful::Model,
        stateless: &stateless::Model,
        args: UpdateArgs,
    ) where
        Comm: CommunicatorCollectives,
    {
        let _local_map = car_map::CarMap::generate(
            &stateless.city.board,
            &stateful.cars[..],
            &stateless.cars[..],
        );
        log::trace!("update cars triggered: {:?}", args);
    }

    pub fn update_city<Comm>(
        &mut self,
        root: Rank,
        communicator: Comm,
        stateful: &mut stateful::City,
        stateless: &stateless::City,
        args: UpdateArgs,
    ) where
        Comm: CommunicatorCollectives,
    {
        if communicator.rank() == root {
            // Update intersection first
            for (stateful_intersection, stateless_intersection) in stateful
                .board
                .intersections
                .iter_mut()
                .zip(stateless.board.intersections.iter())
            {
                if let Some(stateful_intersection) = stateful_intersection.as_mut() {
                    let stateless_intersection = stateless_intersection.as_ref().unwrap();
                    self.update_intersection(stateful_intersection, stateless_intersection, args);
                    stateful_intersection.update_current(stateless_intersection);
                }
            }
        }
        let root_process = communicator.process_at_rank(root);
        communication::bincode_broadcast(communicator.rank(), root_process, stateful).unwrap();
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
