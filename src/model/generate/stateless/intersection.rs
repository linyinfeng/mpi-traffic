use crate::model::board::Board;
use crate::model::board::IntersectionContext;
use crate::model::board::IntersectionIndex;
use crate::model::common::AbsoluteDirection;
use crate::model::stateless::Intersection;
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
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    context: &IntersectionContext,
) {
    //use crate::model::common::AbsoluteDirection::*;
    let _direction_with_road = AbsoluteDirection::directions()
        .find(|&&direction| context.get(direction).is_some())
        .unwrap();
    //    let index = match context.get(direction_with_road) {
    //        Some(index) =>
    //    }
    //    let road = match direction_with_road {
    //        North | South => &board.vertical_roads[index],
    //        East | West => &board.horizontal_roads[index]
    //    };
    //    board.intersections[index] = Some(Intersection::End);
    unimplemented!()
}

pub fn generate_with_2_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    context: &IntersectionContext,
) {
    //use crate::model::common::AbsoluteDirection::*;
    let _directions_with_road = AbsoluteDirection::directions()
        .filter(|&&direction| context.get(direction).is_some())
        .collect::<Vec<&AbsoluteDirection>>();
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
