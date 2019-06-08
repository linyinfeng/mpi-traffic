use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::model::board::Board;
use crate::model::common::TurnRule;
use crate::model::stateless::Lane;
use crate::model::stateless::{Intersection, Road};

pub const MAX_SPEED: f64 = 60.0;

pub fn generate_roads(board: &mut Board<Option<Intersection>, Option<Road>>) {
    generate_basic_board(board);
    mutate_board(board);
}

fn generate_basic_board(board: &mut Board<Option<Intersection>, Option<Road>>) {
    let basic_lane = Lane {
        max_speed: MAX_SPEED,
        direction_rule: TurnRule::ALL,
    };
    let basic_road = Road {
        lane_to_high: vec![basic_lane.clone(), basic_lane.clone()],
        lane_to_low: vec![basic_lane.clone(), basic_lane.clone()],
    };
    board
        .roads_mut()
        .for_each(|(_, road)| *road = Some(basic_road.clone()));
}

fn mutate_board(board: &mut Board<Option<Intersection>, Option<Road>>) {
    board.roads_mut().for_each(|(_, road)| mutate_road(road))
}

fn mutate_road(road: &mut Option<Road>) {
    let road_data = road.as_mut().unwrap(); // no panic here
    mutate_lanes(&mut road_data.lane_to_low);
    mutate_lanes(&mut road_data.lane_to_high);
    if road_data.lane_number() == 0 {
        *road = None;
    }
}

enum MutateStrategy {
    IncreaseRoads(usize),
    DecreaseRoads(usize),
    Nothing,
}

impl Distribution<MutateStrategy> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MutateStrategy {
        match rng.gen_range(0, 3) {
            0 => MutateStrategy::Nothing,
            1 => MutateStrategy::IncreaseRoads(rng.gen_range(1, 3)),
            2 => MutateStrategy::DecreaseRoads(rng.gen_range(1, 3)),
            _ => unreachable!(),
        }
    }
}

fn mutate_lanes(lanes: &mut Vec<Lane>) {
    use MutateStrategy::*;
    match rand::random::<MutateStrategy>() {
        Nothing => (),
        IncreaseRoads(num) => lanes.extend(vec![
            Lane {
                max_speed: MAX_SPEED,
                direction_rule: TurnRule::ALL,
            };
            num
        ]),
        DecreaseRoads(num) => (0..num).for_each(|_| {
            lanes.pop();
        }),
    }
}
