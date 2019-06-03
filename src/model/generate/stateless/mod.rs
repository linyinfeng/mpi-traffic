use crate::model::board::Board;
use crate::model::common::TurnRule;
use crate::model::stateless::Car;
use crate::model::stateless::City;
use crate::model::stateless::Lane;
use crate::model::stateless::Model;
use crate::model::stateless::Road;

pub mod intersection;

pub fn example() -> Result<Model, crate::Error> {
    Ok(Model {
        city: example_city()?,
        cars: example_cars(),
    })
}

pub fn example_city() -> Result<City, crate::Error> {
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

    intersection::generate_from_roads(&mut board)?;
    Ok(City {
        board,
        lane_width: 3.5, // National standard of city road
        // All roads in the example city are 1 km long.
        horizontal_road_length: vec![10000.0, 10000.0, 10000.0],
        vertical_road_length: vec![10000.0, 10000.0, 10000.0],
    })
}

pub fn example_cars() -> Vec<Car> {
    // TODO: Create example cars
    Vec::new()
}
