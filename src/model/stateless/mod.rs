//! Module `stateless` is the static part of the simulation

pub mod car;
pub mod intersection;
pub mod road;

use crate::model::board::Board;
use crate::model::common::DirectionRule;
use car::Car;
use intersection::Intersection;
use road::Road;

#[derive(Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
    pub lane_width: f64,
    pub horizontal_road_length: Vec<f64>,
    pub vertical_road_length: Vec<f64>,
}

impl Board<Option<Intersection>, Option<Road>> {
    pub fn generate_intersections(&mut self) {
        let (_m, _n) = self.intersections.shape();
        for (_i, _j) in self.intersections.indices() {
            unimplemented!()
        }
    }
}

impl City {
    /// Create example city.
    pub fn example() -> Self {
        // TODO: Remove this annotation
        let mut board: Board<Option<Intersection>, Option<Road>> =
            Board::with_shape(None, None, (3, 3));
        let max_speed = 60.0;
        let lane = road::Lane {
            max_speed,
            direction_rule: DirectionRule::ALL, // not properly set
        };
        let two_lane_road = road::Road {
            lane_to_high: vec![lane.clone()],
            lane_to_low: vec![lane.clone()],
        };
        let four_lane_road = road::Road {
            lane_to_high: vec![lane.clone(), lane.clone()],
            lane_to_low: vec![lane.clone(), lane.clone()],
        };
        // create roads
        board.horizontal_roads[(0, 0)] = Some(two_lane_road.clone());
        board.horizontal_roads[(1, 0)] = Some(four_lane_road.clone());
        board.horizontal_roads[(1, 1)] = Some(four_lane_road.clone());
        board.horizontal_roads[(2, 1)] = Some(road::Road {
            lane_to_high: vec![lane.clone()],
            lane_to_low: Vec::new(),
        });
        board.vertical_roads[(0, 0)] = Some(two_lane_road.clone());
        board.vertical_roads[(0, 1)] = Some(two_lane_road.clone());
        board.vertical_roads[(0, 2)] = Some(two_lane_road.clone());
        board.vertical_roads[(1, 1)] = Some(two_lane_road.clone());
        board.vertical_roads[(1, 2)] = Some(two_lane_road.clone());

        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Car>,
}

impl Model {
    pub fn example() -> Self {
        Self {
            city: City::example(),
            cars: Vec::new(),
        }
    }
}
