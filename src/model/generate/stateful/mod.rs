use crate::model::board::Board;
use crate::model::stateful::{City, Model};
use crate::model::stateless;

pub fn generate_from_stateless(stateless_model: &stateless::Model) -> Model {
    stub(stateless_model)
}

fn stub(stateless_model: &stateless::Model) -> Model {
    let board = Board::with_shape(None, None, stateless_model.city.board.intersections.shape());
    Model {
        city: City { board },
        cars: Vec::new(),
    }
}
