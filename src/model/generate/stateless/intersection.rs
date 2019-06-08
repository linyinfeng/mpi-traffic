use crate::model::board::Board;
use crate::model::board::IntersectionContext;
use crate::model::board::IntersectionIndex;
use crate::model::common::AbsoluteDirection;
use crate::model::stateless::Intersection;
use crate::model::stateless::Road;

pub fn generate_from_roads(board: &mut Board<Option<Intersection>, Option<Road>>) {
    board
        .intersections
        .indices()
        .map(|index| (index, board.context_of_intersection(index)))
        .for_each(|(index, context)| board[index] = generate_with_context(&context, board));
}

pub fn generate_with_context(
    context: &IntersectionContext,
    _board: &Board<Option<Intersection>, Option<Road>>,
) -> Option<Intersection> {
    match context.road_number() {
        0 => None,
        1 => Some(Intersection::End),
        2 => unimplemented!(),
        3 => unimplemented!(),
        4 => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn generate_with_1_road(
    board: &mut Board<Option<Intersection>, Option<Road>>,
    index: IntersectionIndex,
    context: &IntersectionContext,
) {
    board.intersections[index] = Some(Intersection::End);
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
