use crate::model::board::Board;
use crate::model::common::{LaneDirection, TurnRule};
use crate::model::generate::stateless::StatelessModelGenerationSettings;
use crate::model::stateless::Lane;
use crate::model::stateless::{Intersection, Road};

pub fn generate_roads(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    generation_settings: &StatelessModelGenerationSettings,
) {
    generate_basic_board(board, generation_settings);
    mutate_board(board, generation_settings);
}

#[inline]
pub fn basic_lane(max_speed: f64) -> Lane {
    Lane {
        max_speed,
        direction_rule: TurnRule::ALL,
    }
}

pub fn basic_road(max_speed: f64, lane_num: usize) -> Road {
    Road {
        lane_to_high: vec![basic_lane(max_speed); lane_num],
        lane_to_low: vec![basic_lane(max_speed); lane_num],
    }
}

fn generate_basic_board(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    generation_settings: &StatelessModelGenerationSettings,
) {
    board.roads_mut().for_each(|(_, road)| {
        *road = Some(basic_road(
            generation_settings.lane_width,
            generation_settings.default_lane_num,
        ))
    });
}

fn mutate_board(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    generation_settings: &StatelessModelGenerationSettings,
) {
    generate_one_way(board, generation_settings);
    remove_road(board, generation_settings);
    add_straight_long_way(board, generation_settings);
}

fn remove_road(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    settings: &StatelessModelGenerationSettings,
) {
    board
        .roads_mut()
        .filter(|(_, road)| road.is_some() && rand::random::<f64>() < settings.empty_proportion)
        .for_each(|(_, road)| {
            road.take();
        })
}

fn generate_one_way(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    settings: &StatelessModelGenerationSettings,
) {
    board
        .roads_mut()
        .filter(|(_, road)| road.is_some() && rand::random::<f64>() < settings.one_way_proportion)
        .for_each(|(_, road)| {
            convert_to_one_way(road.as_mut().unwrap(), settings);
        });
}

fn convert_to_one_way(road: &mut Road, settings: &StatelessModelGenerationSettings) {
    let one_way_direction = rand::random::<LaneDirection>();
    road.lanes_to_direction_mut(one_way_direction.opposite())
        .clear();
    let lanes = &mut road.lanes_to_direction_mut(one_way_direction);
    fix_lanes_num(lanes, settings.one_way_lane_num, settings.lane_max_speed);
}

fn fix_lanes_num(lanes: &mut Vec<Lane>, lanes_num: usize, lane_max_speed: f64) {
    if lanes.len() != lanes_num {
        if lanes_num > lanes.len() {
            lanes.extend(vec![basic_lane(lane_max_speed); lanes_num - lanes.len()]);
        } else {
            for _ in 0..lanes.len() - lanes_num {
                lanes.pop();
            }
        }
    }
}

fn add_straight_long_way(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    settings: &StatelessModelGenerationSettings,
) {
    let (horizontal_row, horizontal_col) = board.horizontal_roads.shape();
    let (vertical_row, vertical_col) = board.vertical_roads.shape();
    (0..horizontal_row)
        .filter(|_| rand::random::<f64>() < settings.straight_long_way_proportion)
        .for_each(|row| {
            (0..horizontal_col).for_each(|col| {
                convert_to_straight_long_way(&mut board.horizontal_roads[(row, col)], settings);
            })
        });
    (0..vertical_col)
        .filter(|_| rand::random::<f64>() < settings.straight_long_way_proportion)
        .for_each(|col| {
            (0..vertical_row).for_each(|row| {
                convert_to_straight_long_way(&mut board.vertical_roads[(row, col)], settings);
            })
        });
}

fn convert_to_straight_long_way(
    road: &mut Option<Road>,
    settings: &StatelessModelGenerationSettings,
) {
    if road.is_none() {
        *road = Some(basic_road(
            settings.lane_max_speed,
            settings.default_lane_num,
        ));
    }
    fix_lanes_num(
        &mut road.as_mut().unwrap().lane_to_low,
        settings.straight_long_way_lane_num,
        settings.lane_max_speed,
    );
    fix_lanes_num(
        &mut road.as_mut().unwrap().lane_to_high,
        settings.straight_long_way_lane_num,
        settings.lane_max_speed,
    );
}
