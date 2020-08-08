use crate::model::{
    board::{Board, IntersectionIndex, RoadIndex},
    common::{
        AbsoluteDirection, Around, AxisDirection, CarIndex, InOutDirection, LaneDirection,
        LaneIndex,
    },
    stateful, stateless,
};

#[derive(Clone, Debug)]
pub struct Road {
    pub lane_to_high: Vec<Lane>,
    pub lane_to_low: Vec<Lane>,
}

impl Road {
    pub fn lanes_to_direction(&self, lane_direction: LaneDirection) -> &Vec<Lane> {
        match lane_direction {
            LaneDirection::HighToLow => &self.lane_to_low,
            LaneDirection::LowToHigh => &self.lane_to_high,
        }
    }

    pub fn lanes_to_direction_mut(&mut self, lane_direction: LaneDirection) -> &mut Vec<Lane> {
        match lane_direction {
            LaneDirection::HighToLow => &mut self.lane_to_low,
            LaneDirection::LowToHigh => &mut self.lane_to_high,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Lane {
    pub cars: Vec<(f64, CarIndex)>, // (position, car_index)
}

impl Lane {
    pub fn sort(&mut self) {
        self.cars
            .sort_by(|(p1, _), (p2, _)| p1.partial_cmp(p2).unwrap());
    }
}

#[derive(Clone, Debug)]
pub struct ProcessLocalState {
    pub board: Board<(), Option<Road>>,
    pub car_out_intersection_lane_out_availability: Around<Vec<bool>>,
}

impl ProcessLocalState {
    pub fn empty(
        board: &Board<Option<stateless::Intersection>, Option<stateless::Road>>,
        car_out_intersection: IntersectionIndex,
    ) -> Self {
        let mut empty_board = Board::with_shape((), None, board.shape());
        for road_direction in AxisDirection::directions() {
            for (road_index, road) in board.get_roads(*road_direction).enumerate() {
                if let Some(road) = road {
                    *empty_board
                        .get_road_mut(*road_direction, road_index)
                        .unwrap() = Some(Road {
                        lane_to_high: vec![Default::default(); road.lane_to_high.len()],
                        lane_to_low: vec![Default::default(); road.lane_to_low.len()],
                    })
                }
            }
        }
        let context = board.context_of_intersection(car_out_intersection);
        let mut car_out_intersection_lane_out_availability: Around<Vec<bool>> = Default::default();
        for direction in AbsoluteDirection::directions() {
            if let Some(road_index) = context.get(*direction) {
                let road = board
                    .get_road(direction.axis_direction(), *road_index)
                    .unwrap()
                    .as_ref()
                    .unwrap();
                let lane_direction =
                    LaneDirection::absolute_in_out_to_lane(*direction, InOutDirection::Out);
                *car_out_intersection_lane_out_availability.get_mut(*direction) =
                    vec![true; road.lanes_to_direction(lane_direction).len()];
            }
        }
        ProcessLocalState {
            board: empty_board,
            car_out_intersection_lane_out_availability,
        }
    }

    pub fn generate(
        city: &stateless::City,
        stateful: &[Option<stateful::Car>],
        _stateless: &[stateless::Car],
    ) -> Self {
        let mut local_state = Self::empty(&city.board, city.car_out_intersection);
        let car_out_intersection_context = city
            .board
            .context_of_intersection(city.car_out_intersection);
        for (i, car) in stateful.iter().enumerate() {
            if let Some(car) = car {
                match car.location {
                    stateful::car::Location::OnLane {
                        road_direction,
                        road_index,
                        lane_direction,
                        lane_index,
                        position,
                        ..
                    } => {
                        local_state.insert_car(
                            road_direction,
                            road_index,
                            lane_direction,
                            lane_index,
                            position,
                            i,
                        );
                        for direction in AbsoluteDirection::directions() {
                            if let Some(out_road_index) =
                                car_out_intersection_context.get(*direction)
                            {
                                let out_road_direction = direction.axis_direction();
                                let out_lane_direction = LaneDirection::absolute_in_out_to_lane(
                                    *direction,
                                    InOutDirection::Out,
                                );
                                if out_road_direction == road_direction
                                    && *out_road_index == road_index
                                    && out_lane_direction == lane_direction
                                    && position < city.car_out_min_distance
                                {
                                    local_state
                                        .car_out_intersection_lane_out_availability
                                        .get_mut(*direction)[lane_index] = false;
                                }
                            }
                        }
                    }
                    stateful::car::Location::ChangingLane {
                        road_direction,
                        road_index,
                        lane_direction,
                        from_lane_index,
                        to_lane_index,
                        position,
                        ..
                    } => {
                        local_state.insert_car(
                            road_direction,
                            road_index,
                            lane_direction,
                            from_lane_index,
                            position,
                            i,
                        );
                        local_state.insert_car(
                            road_direction,
                            road_index,
                            lane_direction,
                            to_lane_index,
                            position,
                            i,
                        )
                    }
                    stateful::car::Location::InIntersection {
                        intersection_index,
                        to_direction,
                        to_lane_index,
                        ..
                    } => {
                        if intersection_index == city.car_out_intersection {
                            local_state
                                .car_out_intersection_lane_out_availability
                                .get_mut(to_direction)[to_lane_index] = false;
                        }
                    }
                }
            }
        }
        local_state.sort_all();
        local_state
    }

    pub fn insert_car(
        &mut self,
        road_direction: AxisDirection,
        road_index: RoadIndex,
        lane_direction: LaneDirection,
        lane_index: LaneIndex,
        position: f64,
        car_index: usize,
    ) {
        self.board
            .get_road_mut(road_direction, road_index)
            .unwrap()
            .as_mut()
            .unwrap()
            .lanes_to_direction_mut(lane_direction)[lane_index]
            .cars
            .push((position, car_index));
    }

    pub fn sort_all(&mut self) {
        for road_direction in AxisDirection::directions() {
            for road in self
                .board
                .get_roads_mut(*road_direction)
                .iter_mut()
                .filter_map(|o| o.as_mut())
            {
                for lane_direction in LaneDirection::directions() {
                    road.lanes_to_direction_mut(*lane_direction)
                        .iter_mut()
                        .for_each(|lane| lane.sort())
                }
            }
        }
    }
}
