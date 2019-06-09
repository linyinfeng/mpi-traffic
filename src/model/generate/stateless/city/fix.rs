use crate::model::board::{Board, IntersectionContext};
use crate::model::common::LaneDirection;
use crate::model::common::TurnRule;
use crate::model::common::{AbsoluteDirection, InOutDirection};
use crate::model::generate::stateless::city::road::basic_lane;
use crate::model::stateless::{Intersection, Lane, Road};

#[derive(Clone, Copy, Debug)]
pub enum LaneAddStrategy {
    Base, // add lane to a existing road
}

pub fn fix(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    lane_add_strategy: LaneAddStrategy,
) {
    fix_intersection_connectivity(board, lane_add_strategy);
    fix_lane_direction_rule(board);
}

fn fix_intersection_connectivity(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    lane_add_strategy: LaneAddStrategy,
) {
    let intersections_need_add_line = board
        .intersections
        .enumerate()
        .filter(|(_, intersection)| intersection.is_some())
        .filter_map(|(index, _)| {
            let context = board.context_of_intersection(index);
            need_lane(board, context).and_then(|in_out_direction| Some((context, in_out_direction)))
        })
        .collect::<Vec<_>>();
    let lane_adder = create_lane_adder(lane_add_strategy);
    intersections_need_add_line
        .into_iter()
        .for_each(|(context, in_out_direction)| {
            lane_adder.add_lane(board, context, in_out_direction)
        })
}

fn need_lane(
    board: &Board<Option<Intersection>, Option<Road>>,
    context: IntersectionContext,
) -> Option<InOutDirection> {
    use std::collections::HashMap;
    use InOutDirection::*;
    let mut need_in_out = HashMap::new();
    need_in_out.insert(In, true);
    need_in_out.insert(Out, true);

    for &direction in AbsoluteDirection::directions() {
        let index = match context.get(direction) {
            &Some(index) => index,
            None => continue,
        };
        let road = board.get_roads(direction.axis_direction())[index]
            .as_ref()
            .unwrap();
        for &lane_direction in LaneDirection::lane_directions() {
            if !road.lanes_to_direction(lane_direction).is_empty() {
                need_in_out.insert(InOutDirection::in_or_out(direction, lane_direction), false);
            }
        }
    }
    need_in_out
        .into_iter()
        .find_map(|(in_out, need_add)| if need_add { Some(in_out) } else { None })
}

fn create_lane_adder(lane_add_strategy: LaneAddStrategy) -> Box<dyn AddLane> {
    use LaneAddStrategy::*;
    match lane_add_strategy {
        Base => Box::new(BaseLaneAdd {}),
    }
}

trait AddLane {
    fn add_lane(
        &self,
        board: &mut Board<Option<Intersection>, Option<Road>>,
        context: IntersectionContext,
        in_out_direction: InOutDirection,
    );
}

struct BaseLaneAdd {}

impl AddLane for BaseLaneAdd {
    fn add_lane(
        &self,
        board: &mut Board<Option<Intersection>, Option<Road>>,
        context: IntersectionContext,
        in_out_direction: InOutDirection,
    ) {
        let (index, direction) = AbsoluteDirection::directions()
            .find_map(|&direction| {
                context
                    .get(direction)
                    .and_then(|index| Some((index, direction)))
            })
            .unwrap();
        let lane_direction = LaneDirection::absolute_in_out_to_lane(direction, in_out_direction);
        board.get_roads_mut(direction.axis_direction())[index]
            .as_mut()
            .unwrap()
            .lanes_to_direction_mut(lane_direction)
            .push(basic_lane());
    }
}

// TODO fix more
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
