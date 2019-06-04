use crate::model::board::Board;
use crate::model::board::IntersectionContext;
use crate::model::board::IntersectionIndex;
use crate::model::common::AbsoluteDirection;
use crate::model::common::TurnRule;
use crate::model::stateless::Intersection;
use crate::model::stateless::Lane;
use crate::model::stateless::Road;

pub fn generate_from_roads(board: &mut Board<Option<Intersection>, Option<Road>>) {
    for index in board.intersections.indices() {
        let context = board.context_of_intersection(index);
        match context.road_number() {
            0 => (),
            1 => generate_with_1_road(board, index, &context),
            2 => generate_with_2_road(board, index, &context),
            3 => generate_with_3_road(board, index, &context),
            4 => generate_with_4_road(board, index, &context),
            _ => unreachable!(),
        }
    }
}

pub fn generate_with_1_road(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    index: IntersectionIndex,
    context: &IntersectionContext,
) {
    board.intersections[index] = Some(Intersection::End);
    fix_end_intersection(board, context)
}

fn fix_end_intersection(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    context: &IntersectionContext,
) {
    use crate::model::common::AbsoluteDirection::*;
    use crate::model::common::AxisDirection::*;
    let direction_with_road = AbsoluteDirection::directions()
        .find(|&&direction| context.get(direction).is_some())
        .unwrap();
    let index = context.get(*direction_with_road).unwrap();
    let road = match direction_with_road.axis_direction() {
        Vertical => &board.vertical_roads[index],
        Horizontal => &board.vertical_roads[index],
    }
    .as_mut()
    .unwrap();
    road.empty_side().and_then(|road| {
        road.push(Lane {
            max_speed,
            direction_rule: TurnRule::ALL,
        })
    });
}

pub fn generate_with_2_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    context: &IntersectionContext,
) {
    let _directions_with_road = AbsoluteDirection::directions()
        .filter(|&&direction| context.get(direction).is_some())
        .map(|&direction| direction)
        .collect::<Vec<AbsoluteDirection>>();
    unimplemented!()
}

pub fn generate_with_3_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    _context: &IntersectionContext,
) {
    unimplemented!()
}

pub fn generate_with_4_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    _context: &IntersectionContext,
) {
    unimplemented!()
}
