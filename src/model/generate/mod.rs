use crate::model::board::Board;
use crate::model::generate::stateful::generate_from_stateless;
use crate::model::generate::stateless::generate_stateless_model;
use crate::model::Model;
use crate::model::stateful as m_stateful;
use crate::model::stateless as m_stateless;
use crate::util::matrix::MatrixShape;

pub mod stateful;
pub mod stateless;

pub fn example() -> Result<(m_stateless::Model, m_stateful::Model), crate::Error> {
    let stateless = stateless::example();
    // TODO: generate_from_stateless unfinished
    //    let stateful = stateful::generate_from_stateless(&stateless);
    let stateful = m_stateful::Model {
        city: m_stateful::City {
            board: Board::with_shape(None, None, (3, 3)),
        },
        cars: Vec::new(),
    };
    Ok((stateless, stateful))
}

// TODO design generation external configuration
pub fn generate_model(board_shape: MatrixShape) -> Model {
    let stateless_model = generate_stateless_model(board_shape);
    let stateful_model = generate_from_stateless(&stateless_model);
    Model {
        stateless: stateless_model,
        stateful: stateful_model,
    }
}