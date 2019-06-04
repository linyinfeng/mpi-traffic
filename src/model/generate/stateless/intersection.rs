use crate::model::board::Board;
use crate::model::board::IntersectionContext;
use crate::model::board::IntersectionIndex;
use crate::model::stateless::Intersection;
use crate::model::stateless::Road;

pub fn generate_from_roads(
    board: &mut Board<Option<Intersection>, Option<Road>>,
) -> Result<(), crate::Error> {
    for index in board.intersections.indices() {
        let context = board.context_of_intersection(index);
        match context.road_number() {
            0 => Ok(()),
            1 => generate_with_1_road(board, index, &context),
            2 => generate_with_2_road(board, index, &context),
            3 => generate_with_3_road(board, index, &context),
            _ => Err(crate::Error::NoIntersectionPattern(context)),
        }?
    }
    Ok(())
}

pub fn generate_with_1_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    _context: &IntersectionContext,
) -> Result<(), crate::Error> {
    unimplemented!()
}

pub fn generate_with_2_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    _context: &IntersectionContext,
) -> Result<(), crate::Error> {
    unimplemented!()
}
pub fn generate_with_3_road(
    _board: &mut Board<Option<Intersection>, Option<Road>>,
    _index: IntersectionIndex,
    _context: &IntersectionContext,
) -> Result<(), crate::Error> {
    unimplemented!()
}
