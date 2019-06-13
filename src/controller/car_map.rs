use crate::model::{
    board::{Board, RoadIndex},
    common::{AxisDirection, CarIndex, LaneDirection, LaneIndex},
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
pub struct CarMap {
    pub board: Board<(), Option<Road>>,
}

impl CarMap {
    pub fn empty(
        board: &Board<Option<stateless::Intersection>, Option<stateless::Road>>,
    ) -> CarMap {
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
        CarMap { board: empty_board }
    }

    pub fn generate(
        board: &Board<Option<stateless::Intersection>, Option<stateless::Road>>,
        stateful: &[Option<stateful::Car>],
        _stateless: &[stateless::Car],
    ) -> Self {
        let mut map = Self::empty(board);
        for (i, car) in stateful.iter().enumerate() {
            if let Some(car) = car {
                match car.location {
                    stateful::car::Location::OnLane {
                        road_direction,
                        road_index,
                        lane_direction,
                        lane_index,
                        position,
                    } => map.insert_car(
                        road_direction,
                        road_index,
                        lane_direction,
                        lane_index,
                        position,
                        i,
                    ),
                    stateful::car::Location::ChangingLane {
                        road_direction,
                        road_index,
                        lane_direction,
                        from_lane_index,
                        to_lane_index,
                        position,
                        ..
                    } => {
                        map.insert_car(
                            road_direction,
                            road_index,
                            lane_direction,
                            from_lane_index,
                            position,
                            i,
                        );
                        map.insert_car(
                            road_direction,
                            road_index,
                            lane_direction,
                            to_lane_index,
                            position,
                            i,
                        )
                    },
                    _ => (),
                }
            }
        }
        map.sort_all();
        map
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
