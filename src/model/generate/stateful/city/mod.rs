use crate::model::{board::Board, stateful::City, stateless};

pub mod intersection;
pub mod road;

pub fn generate_city_from_stateless(stateless_model: &stateless::City) -> City {
    let mut city = City {
        board: Board::with_shape(None, None, stateless_model.board.shape()),
    };
    for index in stateless_model.board.intersections.indices() {
        if let Some(stateless_intersection) = &stateless_model.board.intersections[index] {
            city.board.intersections[index] = Some(
                intersection::generate_intersection_from_stateless(stateless_intersection),
            );
        }
    }
    city
}
