use crate::model::board::Board;
use crate::model::stateless::Car;
use crate::model::stateless::City;
use crate::model::stateless::Model;
use crate::util::matrix::MatrixShape;

mod car;
mod fix;
mod intersection;
mod road;

pub fn generate_stateless_model(board_shape: MatrixShape) -> Model {
    Model {
        city: generate_city(board_shape),
        cars: car::generate_cars(),
    }
}

fn generate_city(board_shape: MatrixShape) -> City {
    let mut board = Board::with_shape(None, None, board_shape);
    road::generate_roads(&mut board);
    intersection::generate_intersections(&mut board);
    fix::fix(&mut board);
    City {
        board,
        // TODO use external settings
        lane_width: 3.5, // National standard of city road
        // All roads in the example city are 500 m long.
        horizontal_road_length: vec![100.0, 100.0],
        vertical_road_length: vec![100.0, 100.0],
        intersection_height: vec![20.0, 20.0, 20.0], // TODO: Can be generated
        intersection_width: vec![20.0, 20.0, 20.0],  // TODO: Can be generated
    }
}

pub fn example() -> Model {
    Model {
        city: example_city(),
        cars: example_cars(),
    }
}

fn example_city() -> City {
    use crate::model::common::TurnRule;
    use crate::model::stateless::{Lane, Road};
    let mut board = Board::with_shape(None, None, (3, 3));
    let max_speed = 60.0;
    let lane = Lane {
        max_speed,
        direction_rule: TurnRule::ALL, // not properly set
    };
    let two_lane_road = Road {
        lane_to_high: vec![lane.clone()],
        lane_to_low: vec![lane.clone()],
    };
    let four_lane_road = Road {
        lane_to_high: vec![lane.clone(), lane.clone()],
        lane_to_low: vec![lane.clone(), lane.clone()],
    };
    // Create roads
    board.horizontal_roads[(0, 0)] = Some(two_lane_road.clone());
    board.horizontal_roads[(1, 0)] = Some(four_lane_road.clone());
    board.horizontal_roads[(1, 1)] = Some(four_lane_road.clone());
    board.horizontal_roads[(2, 1)] = Some(Road {
        lane_to_high: vec![lane.clone()],
        lane_to_low: Vec::new(),
    });
    board.vertical_roads[(0, 0)] = Some(two_lane_road.clone());
    board.vertical_roads[(0, 1)] = Some(two_lane_road.clone());
    board.vertical_roads[(0, 2)] = Some(two_lane_road.clone());
    board.vertical_roads[(1, 1)] = Some(two_lane_road.clone());
    board.vertical_roads[(1, 2)] = Some(two_lane_road.clone());
    intersection::generate_intersections(&mut board);

    City {
        board,
        lane_width: 3.5, // National standard of city road
        // All roads in the example city are 500 m long.
        horizontal_road_length: vec![100.0, 100.0],
        vertical_road_length: vec![100.0, 100.0],
        intersection_height: vec![20.0, 20.0, 20.0], // TODO: Can be generated
        intersection_width: vec![20.0, 20.0, 20.0],  // TODO: Can be generated
    }
}

fn example_cars() -> Vec<Car> {
    // TODO: Create example cars
    Vec::new()
}
