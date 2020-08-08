use rand::{self, Rng};

use crate::model::{
    board::{Board, IntersectionIndex},
    generate::stateless::StatelessModelGenerationSettings,
    stateless::{City, Intersection, Road},
};

mod fix;
pub mod intersection;
pub mod road;

pub const MIN_LANE_LENGTH: f64 = 50.0;
pub const MAX_LANE_LENGTH: f64 = 100.0;

pub fn generate_city(city_settings: &StatelessModelGenerationSettings) -> City {
    let board_shape = (
        city_settings.board_shape_rows,
        city_settings.board_shape_cols,
    );
    let mut board = Board::with_shape(None, None, board_shape);
    road::generate_roads(&mut board, &city_settings);
    intersection::generate_intersections(&mut board, &city_settings);
    fix::fix(&mut board, &city_settings);

    let (intersection_height, intersection_width) =
        calculate_intersection_geometry(&board, city_settings.lane_width);
    let car_out_intersection = generate_car_out_intersection(&board, city_settings);
    City {
        board,
        car_out_intersection,
        car_out_min_distance: city_settings.car_out_min_distance,
        lane_width: city_settings.lane_width,
        horizontal_road_length: rand_road_length(board_shape.1 - 1, city_settings),
        vertical_road_length: rand_road_length(board_shape.0 - 1, city_settings),
        intersection_height,
        intersection_width,
    }
}

fn generate_car_out_intersection(
    board: &Board<Option<Intersection>, Option<Road>>,
    _city_settings: &StatelessModelGenerationSettings,
) -> IntersectionIndex {
    for (index, intersection) in board.intersections.enumerate() {
        if intersection.is_some() {
            return index;
        }
    }
    panic!("empty city")
}

fn rand_road_length(road_num: usize, settings: &StatelessModelGenerationSettings) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..road_num)
        .map(|_| rng.gen_range(settings.min_road_length, settings.max_road_length))
        .collect()
}

fn calculate_intersection_geometry(
    board: &Board<Option<Intersection>, Option<Road>>,
    lane_width: f64,
) -> (Vec<f64>, Vec<f64>) {
    let mut height = vec![0.0; board.intersections.shape().0];
    let mut width = vec![0.0; board.intersections.shape().1];
    board
        .horizontal_roads
        .enumerate()
        .for_each(|(index, road)| {
            let length = road
                .as_ref()
                .map_or(0.0, |road| road.lane_number() as f64 * lane_width);
            if height[index.0] < length {
                height[index.0] = length
            }
        });
    board
        .vertical_roads
        .enumerate()
        .for_each(|(index, intersection)| {
            let length = intersection
                .as_ref()
                .map_or(0.0, |road| road.lane_number() as f64 * lane_width);
            if width[index.1] < length {
                width[index.1] = length
            }
        });
    (height, width)
}
