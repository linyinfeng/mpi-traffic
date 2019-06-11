use crate::model::board::{Board, IntersectionContext};
use crate::model::common::LaneDirection;
use crate::model::common::TurnRule;
use crate::model::common::{AbsoluteDirection, InOutDirection};
use crate::model::generate::stateless::city::road::basic_lane;
use crate::model::generate::stateless::StatelessModelGenerationSettings;
use crate::model::stateless::{Intersection, Lane, Road};

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
        // TODO FIX
        .push(basic_lane(lane_max_speed));
}

fn fix_lane_direction_rule(board: &mut Board<Option<Intersection>, Option<Road>>) {
    for (_, road) in board.roads_mut() {
        if let Some(ref mut road) = road {
            assert!(!road.lane_to_low.is_empty() || !road.lane_to_high.is_empty());
            fix_road_side(&mut road.lane_to_low);
            fix_road_side(&mut road.lane_to_high);
            if road.lane_to_high.is_empty() {
                road.lane_to_low[0].direction_rule -= TurnRule::BACK;
            }
            if road.lane_to_low.is_empty() {
                road.lane_to_high[0].direction_rule -= TurnRule::BACK;
            }
        }
    }
}

fn fix_road_side(lanes: &mut [Lane]) {
    if lanes.len() > 1 {
        lanes[0].direction_rule -= TurnRule::RIGHT;
        lanes[lanes.len() - 1].direction_rule -= TurnRule::LEFT;
    }
}
