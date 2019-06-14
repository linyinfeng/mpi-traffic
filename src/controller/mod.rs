use crate::{
    communication::{self, Division},
    info::Info,
    model::{
        board::RoadIndex,
        common::{
            AbsoluteDirection, AxisDirection, CarIndex,
            InOutDirection::{self, Out},
            LaneDirection, LaneIndex, RelativeDirection, TurnRule,
        },
        stateful::{self, Car},
        stateless::{self, car::DrivingModel},
    },
};
use mpi::{collective::CommunicatorCollectives, topology::Rank};
use piston_window::{Button, ButtonArgs, ButtonState, Input, Motion, MouseButton, UpdateArgs};
use process_local_state::ProcessLocalState;
use rand::{self, Rng};
use structopt::StructOpt;

pub mod process_local_state;

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

#[derive(Clone, Debug, Default)]
pub struct UpdateController {
    car_out_rank: Rank,
}

impl UpdateController {
    pub fn new() -> Self {
        Self { car_out_rank: 0 }
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
        self.update_cars(root, communicator.clone(), stateful, stateless, args);

        self.car_out_rank += 1;
        self.car_out_rank %= communicator.size();
    }

    pub fn update_cars<Comm>(
        &mut self,
        _root: Rank,
        communicator: Comm,
        stateful: &mut stateful::Model,
        stateless: &stateless::Model,
        args: UpdateArgs,
    ) where
        Comm: CommunicatorCollectives,
    {
        let local_state =
            ProcessLocalState::generate(&stateless.city, &stateful.cars[..], &stateless.cars[..]);

        let car_number = stateful.cars.len();
        let rank = communicator.rank();
        let size = communicator.size();
        let division = Division::new(car_number, rank, size);
        let mut local_cars = Vec::new();
        let mut outed = false;
        for car_index in division.range() {
            local_cars.push(self.update_car(
                &mut outed,
                communicator.rank(),
                car_index,
                &local_state,
                &*stateful,
                stateless,
                args,
            ));
        }
        let gathered =
            communication::bincode_all_gather_varcount(communicator, &local_cars).unwrap();
        stateful.cars = gathered.into_iter().flatten().collect();
    }

    pub fn update_car(
        &self,
        outed: &mut bool,
        rank: Rank,
        car_index: CarIndex,
        local_state: &ProcessLocalState,
        stateful: &stateful::Model,
        stateless: &stateless::Model,
        args: UpdateArgs,
    ) -> Option<stateful::Car> {
        use crate::model::stateful::car::Location::*;
        let stateless_car = &stateless.cars[car_index];
        if let Some(car) = &stateful.cars[car_index] {
            match &car.location {
                OnLane {
                    road_direction,
                    road_index,
                    lane_direction,
                    lane_index,
                    about_to_turn,
                    position,
                } => {
                    let road_length = stateless.city.road_length(*road_direction, *road_index);
                    let road = stateless.city.board.get_roads(*road_direction)[*road_index]
                        .as_ref()
                        .unwrap();
                    let lane = &road.lanes_to_direction(*lane_direction)[*lane_index];
                    // let max_velocity = stateless_car.max_velocity.min(lane.max_speed);
                    let max_velocity = stateless_car.max_velocity;
                    let mut velocity =
                        (car.velocity + car.acceleration * args.dt).min(max_velocity);
                    if car.acceleration < 0.0 && velocity < 2.0 * car.acceleration {
                        velocity = 0.0;
                    }
                    let position = position + car.velocity * args.dt;

                    if position >= road_length {
                        // switch to InIntersection
                        let intersection_index = stateless.city.board.lane_to_intersection_index(
                            *road_direction,
                            *road_index,
                            *lane_direction,
                        );
                        let driver_direction =
                            AbsoluteDirection::of_lane(*road_direction, *lane_direction);
                        let to_direction = driver_direction.turn(*about_to_turn);
                        let from_direction = driver_direction.turn_back();
                        let to_lane_index = {
                            let mut rng = rand::thread_rng();
                            let road_index = *stateless
                                .city
                                .board
                                .context_of_intersection(intersection_index)
                                .get(to_direction)
                                .as_ref()
                                .expect("no way to turn");
                            let lane_direction = LaneDirection::absolute_in_out_to_lane(
                                to_direction,
                                InOutDirection::Out,
                            );
                            let road = stateless
                                .city
                                .board
                                .get_road(to_direction.axis_direction(), road_index)
                                .unwrap()
                                .as_ref()
                                .unwrap();
                            let size = road.lanes_to_direction(lane_direction).len();
                            rng.gen_range(0, size)
                        };
                        let total_length = stateless
                            .city
                            .intersection_path_total_length(
                                intersection_index,
                                from_direction,
                                *lane_index,
                                to_direction,
                                to_lane_index,
                            )
                            .unwrap();
                        let location = InIntersection {
                            intersection_index,
                            from_direction,
                            from_lane_index: *lane_index,
                            to_direction,
                            to_lane_index,
                            total_length,
                            position: 0.0,
                        };
                        Some(Car {
                            location,
                            velocity: car.velocity,
                            acceleration: 0.0,
                        })
                    } else {
                        let mut front_objects = Vec::new();
                        {
                            let front_car_index = self.get_front_car(
                                car_index,
                                local_state,
                                *road_direction,
                                *road_index,
                                *lane_direction,
                                *lane_index,
                            );
                            if let Some(front_car_index) = front_car_index {
                                let front_car = stateful.cars[front_car_index].as_ref().unwrap();
                                let front_position = match &front_car.location {
                                    OnLane { position, .. } => *position,
                                    _ => unreachable!(),
                                };
                                let velocity = front_car.velocity;
                                front_objects.push((front_position - position, velocity));
                            }
                        }
                        {
                            let intersection_index =
                                stateless.city.board.lane_to_intersection_index(
                                    *road_direction,
                                    *road_index,
                                    *lane_direction,
                                );
                            let stateful_intersection = stateful.city.board.intersections
                                [intersection_index]
                                .as_ref()
                                .unwrap();
                            let stateless_intersection = stateless.city.board.intersections
                                [intersection_index]
                                .as_ref()
                                .unwrap();
                            match stateless_intersection {
                                stateless::Intersection::Crossroad { max_speed, .. } => {
                                    match stateful_intersection {
                                        stateful::Intersection::Crossroad { current, .. } => {
                                            let from_direction = AbsoluteDirection::of_lane(
                                                *road_direction,
                                                *lane_direction,
                                            )
                                            .turn_back();
                                            let turn_rule = *current.get(from_direction);
                                            if about_to_turn.to_turn_rule().intersects(turn_rule) {
                                                front_objects
                                                    .push((road_length - position, *max_speed))
                                            } else {
                                                front_objects.push((road_length - position, 0.0))
                                            }
                                        },
                                        _ => unreachable!(),
                                    }
                                },
                                stateless::Intersection::TJunction { max_speed, .. } => {
                                    match stateful_intersection {
                                        stateful::Intersection::TJunction { current, .. } => {
                                            // TODO: Fix redundant code
                                            let from_direction = AbsoluteDirection::of_lane(
                                                *road_direction,
                                                *lane_direction,
                                            )
                                            .turn_back();
                                            let turn_rule = *current.get(from_direction);
                                            if about_to_turn.to_turn_rule().intersects(turn_rule) {
                                                front_objects
                                                    .push((road_length - position, *max_speed))
                                            } else {
                                                front_objects.push((road_length - position, 0.0))
                                            }
                                        },
                                        _ => unreachable!(),
                                    }
                                },
                                stateless::Intersection::Turn { max_speed } => {
                                    front_objects.push((road_length - position, *max_speed))
                                },
                                stateless::Intersection::Straight => {
                                    front_objects.push((road_length - position, lane.max_speed))
                                },
                                stateless::Intersection::End { max_speed } => {
                                    front_objects.push((road_length - position, *max_speed))
                                },
                            }
                        }
                        let acceleration = front_objects
                            .into_iter()
                            .map(|(object_distance, object_velocity)| {
                                Self::driver_acceleration(
                                    car.velocity,
                                    car.acceleration,
                                    &stateless_car.driving_model,
                                    object_distance,
                                    object_velocity,
                                )
                            })
                            .min_by(|a, b| a.partial_cmp(b).unwrap())
                            .expect("car can not detect any object front");
                        let acceleration = acceleration
                            .min(stateless_car.max_acceleration)
                            .max(-stateless_car.max_break_acceleration);
                        Some(Car {
                            velocity,
                            acceleration,
                            location: OnLane {
                                road_direction: *road_direction,
                                road_index: *road_index,
                                lane_direction: *lane_direction,
                                lane_index: *lane_index,
                                about_to_turn: *about_to_turn,
                                position,
                            },
                        })
                    }
                },
                InIntersection {
                    intersection_index,
                    from_direction,
                    from_lane_index,
                    to_direction,
                    to_lane_index,
                    total_length,
                    position,
                } => {
                    let stateless_intersection = stateless.city.board.intersections
                        [*intersection_index]
                        .as_ref()
                        .unwrap();
                    let intersection_max_speed = match stateless_intersection {
                        stateless::Intersection::Crossroad { max_speed, .. } => Some(max_speed),
                        stateless::Intersection::TJunction { max_speed, .. } => Some(max_speed),
                        stateless::Intersection::Turn { max_speed } => Some(max_speed),
                        stateless::Intersection::Straight => None,
                        stateless::Intersection::End { max_speed } => Some(max_speed),
                    };
                    let velocity = match intersection_max_speed {
                        Some(max_speed) => {
                            if car.velocity < *max_speed {
                                let dv = max_speed - car.velocity;
                                car.velocity + 0.5 * dv * args.dt
                            } else {
                                car.velocity
                            }
                        },
                        None => car.velocity,
                    };
                    let position = position + car.velocity * args.dt;
                    if position >= *total_length {
                        let context = stateless
                            .city
                            .board
                            .context_of_intersection(*intersection_index);
                        let out_road_index = context.get(*to_direction).unwrap();
                        let to_lane_direction =
                            LaneDirection::absolute_in_out_to_lane(*to_direction, Out);
                        let turn_rule = stateless
                            .city
                            .board
                            .get_roads(to_direction.axis_direction())[out_road_index]
                            .as_ref()
                            .unwrap()
                            .lanes_to_direction(to_lane_direction)[*to_lane_index]
                            .direction_rule;
                        let about_to_turn = self.random_choose_relative_direction(turn_rule);
                        match about_to_turn {
                            Some(about_to_turn) => {
                                let updated_car = OnLane {
                                    road_direction: to_direction.axis_direction(),
                                    road_index: out_road_index,
                                    lane_direction: to_lane_direction,
                                    lane_index: *to_lane_index,
                                    about_to_turn,
                                    position: 0.0,
                                };
                                Some(Car {
                                    location: updated_car,
                                    velocity,
                                    acceleration: 0.0,
                                })
                            },
                            None => None, // remove the car
                        }

                    } else {
                        Some(Car {
                            location: InIntersection {
                                intersection_index: *intersection_index,
                                from_direction: *from_direction,
                                from_lane_index: *from_lane_index,
                                to_direction: *to_direction,
                                to_lane_index: *to_lane_index,
                                total_length: *total_length,
                                position,
                            },
                            velocity: car.velocity,
                            acceleration: 0.0,
                        })
                    }
                },
                _ => unimplemented!(),
            }
        } else if self.car_out_rank == rank && !*outed {
            *outed = true;
            match self.try_out_car(local_state, stateful, stateless) {
                Some((road_direction, road_index, lane_direction, lane_index)) => {
                    let turn_rule = stateless.city.board.get_roads(road_direction)[road_index]
                        .as_ref()
                        .unwrap()
                        .lanes_to_direction(lane_direction)[lane_index]
                        .direction_rule;
                    let about_to_turn = self.random_choose_relative_direction(turn_rule);
                    match about_to_turn {
                        Some(about_to_turn) => {
                            let car = stateful::Car {
                                location: stateful::car::Location::OnLane {
                                    road_direction,
                                    road_index,
                                    lane_direction,
                                    lane_index,
                                    position: 0.0,
                                    about_to_turn,
                                },
                                acceleration: 0.0,
                                velocity: 0.0,
                            };
                            log::debug!("Crate new car: {:?}", car);
                            Some(car)
                        },
                        None => None,
                    }
                },
                None => None,
            }
        } else {
            None
        }
    }

    pub fn driver_acceleration(
        velocity: f64,
        _acceleration: f64,
        driving_model: &DrivingModel,
        front_distance: f64,
        front_velocity: f64,
    ) -> f64 {
        match driving_model {
            DrivingModel::Normal {
                min_cushion,
                cushion_velocity_factor,
                prediction_time,
            } => {
                let aim_cushion = min_cushion + cushion_velocity_factor * front_velocity;
                let dx = front_distance - aim_cushion; // if dx is greater than 0, the car should go faster than front_velocity
                let aim_average_velocity = dx / *prediction_time;
                (aim_average_velocity + front_velocity - velocity) * 2.0 / *prediction_time
            },
        }
    }

    fn get_front_car(
        &self,
        current_car: CarIndex,
        local_state: &ProcessLocalState,
        road_direction: AxisDirection,
        road_index: RoadIndex,
        lane_direction: LaneDirection,
        lane_index: LaneIndex,
    ) -> Option<CarIndex> {
        let lane_cars = &local_state.board.get_roads(road_direction)[road_index]
            .as_ref()
            .unwrap()
            .lanes_to_direction(lane_direction)[lane_index];
        for (index, (_, car_index)) in lane_cars.cars.iter().enumerate() {
            if *car_index == current_car && index != lane_cars.cars.len() - 1 {
                return Some(lane_cars.cars[index + 1].1)
            }
        }
        None
    }

    fn random_choose_relative_direction(&self, turn_rule: TurnRule) -> Option<RelativeDirection> {
        use crate::model::common::RelativeDirection::*;
        let all_rule = [
            TurnRule::FRONT,
            TurnRule::BACK,
            TurnRule::LEFT,
            TurnRule::RIGHT,
        ];
        let enabled_rule = all_rule
            .iter()
            .filter(|&rule| (turn_rule & *rule) == *rule)
            .collect::<Vec<_>>();
        match enabled_rule.len() {
            0 => None,
            len => {
                let mut rng = rand::thread_rng();
                let rule = enabled_rule[rng.gen_range(0usize, len)];
                Some(match *rule {
                    TurnRule::FRONT => Front,
                    TurnRule::BACK => Back,
                    TurnRule::LEFT => Left,
                    TurnRule::RIGHT => Right,
                    _ => unreachable!(),
                })
            }
        }

    }

    pub fn try_out_car(
        &self,
        local_state: &ProcessLocalState,
        _stateful: &stateful::Model,
        stateless: &stateless::Model,
    ) -> Option<(AxisDirection, RoadIndex, LaneDirection, LaneIndex)> {
        log::trace!("try_out_car called");
        let context = stateless
            .city
            .board
            .context_of_intersection(stateless.city.car_out_intersection);
        for direction in AbsoluteDirection::directions() {
            let lanes_availability = local_state
                .car_out_intersection_lane_out_availability
                .get(*direction);
            for (lane_index, availability) in lanes_availability.iter().enumerate() {
                if *availability {
                    let road_index = context.get(*direction).unwrap();
                    let car_out_parameter = (
                        direction.axis_direction(),
                        road_index,
                        LaneDirection::absolute_in_out_to_lane(*direction, InOutDirection::Out),
                        lane_index,
                    );
                    log::debug!("car out parameter: {:?}", car_out_parameter);
                    return Some(car_out_parameter)
                }
            }
        }
        None
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
