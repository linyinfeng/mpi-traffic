use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::model::board::Board;
use crate::model::common::TurnRule;
use crate::model::stateless::Lane;
use crate::model::stateless::{Intersection, Road};

pub const MAX_SPEED: f64 = 60.0;

pub enum GenerationStrategy {
    Random,
}

pub fn generate_roads(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    strategy: GenerationStrategy,
) {
    create_generator(strategy).generate(board)
}

#[inline]
pub fn basic_lane() -> Lane {
    Lane {
        max_speed: MAX_SPEED,
        direction_rule: TurnRule::ALL,
    }
}

pub fn basic_road() -> Road {
    Road {
        lane_to_high: vec![basic_lane(), basic_lane()],
        lane_to_low: vec![basic_lane(), basic_lane()],
    }
}

fn create_generator(strategy: GenerationStrategy) -> Box<dyn Generator> {
    match strategy {
        GenerationStrategy::Random => Box::new(RandomRoadGenerator {}),
    }
}

trait Generator {
    fn generate(&self, board: &mut Board<Option<Intersection>, Option<Road>>);
}

struct RandomRoadGenerator {}

impl Generator for RandomRoadGenerator {
    fn generate(&self, board: &mut Board<Option<Intersection>, Option<Road>>) {
        Self::generate_basic_board(board);
        Self::mutate_board(board);
    }
}

impl RandomRoadGenerator {
    fn generate_basic_board(board: &mut Board<Option<Intersection>, Option<Road>>) {
        let basic_road = basic_road();
        board
            .roads_mut()
            .for_each(|(_, road)| *road = Some(basic_road.clone()));
    }

    fn mutate_board(board: &mut Board<Option<Intersection>, Option<Road>>) {
        board
            .roads_mut()
            .for_each(|(_, road)| Self::mutate_road(road))
    }

    fn mutate_road(road: &mut Option<Road>) {
        let road_data = road.as_mut().unwrap(); // no panic here
        Self::mutate_lanes(&mut road_data.lane_to_low);
        Self::mutate_lanes(&mut road_data.lane_to_high);
        if road_data.lane_number() == 0 {
            *road = None;
        }
    }

    fn mutate_lanes(lanes: &mut Vec<Lane>) {
        use MutateStrategy::*;
        match rand::random::<MutateStrategy>() {
            Nothing => (),
            IncreaseRoads(num) => lanes.extend(vec![basic_lane(); num]),
            DecreaseRoads(num) => (0..num).for_each(|_| {
                lanes.pop();
            }),
        }
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
