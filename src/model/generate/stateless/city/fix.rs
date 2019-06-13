use crate::model::{
    board::{Board, IntersectionContext},
    common::{AbsoluteDirection, InOutDirection, LaneDirection, RelativeDirection, TurnRule},
    generate::stateless::{
        city::{intersection::is_turn_intersection, road::basic_lane},
        StatelessModelGenerationSettings,
    },
    stateless::{Intersection, Lane, Road},
};
use log::debug;

pub fn fix(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    city_settings: &StatelessModelGenerationSettings,
) {
    fix_intersection_connectivity(board, city_settings);
    fix_lane_direction_rule(board);
}

fn fix_intersection_connectivity(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    city_settings: &StatelessModelGenerationSettings,
) {
    let intersections_need_add_line = board
        .intersections
        .indices()
        .map(|index| board.context_of_intersection(index))
        .filter_map(|context| {
            need_lane(board, context).map(|in_out_direction| (context, in_out_direction))
        })
        .collect::<Vec<_>>();
    intersections_need_add_line
        .iter()
        .for_each(|&(context, in_out_direction)| {
            add_lane(
                board,
                context,
                in_out_direction,
                city_settings.lane_max_speed,
            )
        });
}

fn need_lane(
    board: &Board<Option<Intersection>, Option<Road>>,
    context: IntersectionContext,
) -> Option<InOutDirection> {
    use InOutDirection::*;
    let mut need_in = Some(In);
    let mut need_out = Some(Out);
    AbsoluteDirection::directions()
        .filter_map(|&direction| context.get(direction).map(|index| (direction, index)))
        .for_each(|(direction, index)| {
            LaneDirection::lane_directions().for_each(|&lane_direction| {
                if !board.get_roads(direction.axis_direction())[index]
                    .as_ref()
                    .unwrap()
                    .lanes_to_direction(lane_direction)
                    .is_empty()
                {
                    match InOutDirection::in_or_out(direction, lane_direction) {
                        In => need_in = None,
                        Out => need_out = None,
                    };
                };
            })
        });
    need_in.or(need_out)
}

fn add_lane(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    context: IntersectionContext,
    in_out_direction: InOutDirection,
    lane_max_speed: f64,
) {
    let (index, direction) = AbsoluteDirection::directions()
        .filter_map(|&direction| context.get(direction).map(|index| (index, direction)))
        .min_by_key(|&(index, direction)| {
            board.get_roads(direction.axis_direction())[index]
                .as_ref()
                .unwrap()
                .lane_number()
        })
        .unwrap();
    let lane_direction = LaneDirection::absolute_in_out_to_lane(direction, in_out_direction);
    board.get_roads_mut(direction.axis_direction())[index]
        .as_mut()
        .unwrap()
        .lanes_to_direction_mut(lane_direction)
        .push(basic_lane(lane_max_speed));
}

fn fix_lane_direction_rule(board: &mut Board<Option<Intersection>, Option<Road>>) {
    fix_lane_direction_rule_by_road(board);
    fix_lane_direction_rule_by_context(board);
}

fn fix_lane_direction_rule_by_road(board: &mut Board<Option<Intersection>, Option<Road>>) {
    for road in board.roads_mut().filter_map(|(_, road)| road.as_mut()) {
        LaneDirection::lane_directions().for_each(|&lane_direction| {
            let lanes = road.lanes_to_direction_mut(lane_direction);
            let len = lanes.len();
            if len > 1 {
                // only last lane can turn right
                lanes.iter_mut().take(len - 1).for_each(|lane| {
                    lane.direction_rule -= TurnRule::RIGHT;
                });
                // only first len can turn left and back
                lanes
                    .iter_mut()
                    .skip(1)
                    .for_each(|lane| lane.direction_rule -= TurnRule::LEFT | TurnRule::BACK);
            }
        });
        LaneDirection::lane_directions().for_each(|&lane_direction| {
            if road
                .lanes_to_direction(lane_direction.opposite())
                .is_empty()
            {
                road.lanes_to_direction_mut(lane_direction)[0].direction_rule -= TurnRule::BACK;
            }
        })
    }
}

fn fix_lane_direction_rule_by_context(board: &mut Board<Option<Intersection>, Option<Road>>) {
    for intersection_index in board.intersections.indices() {
        fix_lane_rule_with_context(board, &board.context_of_intersection(intersection_index));
    }
}

fn fix_lane_rule_with_context(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    context: &IntersectionContext,
) {
    use InOutDirection::*;
    use RelativeDirection::*;
    let directions_with_in_road = AbsoluteDirection::directions()
        .filter(|&&direction| {
            context.get(direction).is_some() && {
                let road_index = context.get(direction).unwrap();
                !board.get_roads(direction.axis_direction())[road_index]
                    .as_ref()
                    .unwrap()
                    .lanes_to_direction(LaneDirection::absolute_in_out_to_lane(direction, In))
                    .is_empty()
            }
        })
        .collect::<Vec<_>>();
    let directions_without_out_lanes = AbsoluteDirection::directions()
        .filter(|&&direction| {
            context.get(direction).is_none() || {
                let road_index = context.get(direction).unwrap();
                board.get_roads(direction.axis_direction())[road_index]
                    .as_ref()
                    .unwrap()
                    .lanes_to_direction(LaneDirection::absolute_in_out_to_lane(direction, Out))
                    .is_empty()
            }
        })
        .collect::<Vec<_>>();
    debug!("Directions with in road:{:?}", directions_with_in_road);
    debug!(
        "Directions without out road:{:?}",
        directions_without_out_lanes
    );

    for &&direction in directions_with_in_road.iter() {
        let lanes_to_be_fix = find_lanes_to_be_fix(direction, board, context);
        let len = lanes_to_be_fix.len();
        if len == 0 {
            continue
        }
        for &&direction_without_out_lane in directions_without_out_lanes.iter() {
            match direction.should_turn(direction_without_out_lane) {
                Front => (),
                Back => lanes_to_be_fix.iter_mut().for_each(|lane| {
                    debug!(
                        "direction:{:?} FRONT:{:?}",
                        direction, direction_without_out_lane
                    );
                    lane.direction_rule -= TurnRule::FRONT;
                }),
                Right => lanes_to_be_fix[0].direction_rule -= TurnRule::LEFT,
                Left => lanes_to_be_fix[len - 1].direction_rule -= TurnRule::RIGHT,
            }
        }
    }

    if is_turn_intersection(context) {
        debug!("TRUN:{:?}", context);
        for &in_direction in directions_with_in_road.iter() {
            let current_lanes_out_direction = AbsoluteDirection::directions()
                .filter(|direction| {
                    !directions_without_out_lanes.contains(direction) && *direction != in_direction
                })
                .collect::<Vec<_>>();
            assert!(
                current_lanes_out_direction.len() == 1 || current_lanes_out_direction.is_empty()
            );
            if !current_lanes_out_direction.is_empty() {
                let out_direction = current_lanes_out_direction[0];
                let lanes_to_be_fixed = find_lanes_to_be_fix(*in_direction, board, context);
                match in_direction.should_turn(*out_direction) {
                    Front => unreachable!("Front"),
                    Back => unreachable!("Back"),
                    Right => lanes_to_be_fixed
                        .iter_mut()
                        .for_each(|lane| lane.direction_rule |= TurnRule::LEFT),
                    Left => lanes_to_be_fixed
                        .iter_mut()
                        .for_each(|lane| lane.direction_rule |= TurnRule::RIGHT),
                }
            }
        }
    }
}

fn find_lanes_to_be_fix<'a>(
    direction: AbsoluteDirection,
    board: &'a mut Board<Option<Intersection>, Option<Road>>,
    context: &IntersectionContext,
) -> &'a mut Vec<Lane> {
    use InOutDirection::*;
    let in_lane_direction = LaneDirection::absolute_in_out_to_lane(direction, In);
    let road_index = context.get(direction).unwrap();
    board.get_roads_mut(direction.axis_direction())[road_index]
        .as_mut()
        .unwrap()
        .lanes_to_direction_mut(in_lane_direction)
}
