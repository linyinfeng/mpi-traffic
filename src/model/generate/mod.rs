use crate::model::board::Board;
use crate::model::stateful as m_stateful;
use crate::model::stateless as m_stateless;

pub mod stateful;
pub mod stateless;

pub fn example() -> Result<(m_stateless::Model, m_stateful::Model), crate::Error> {
    let stateless = stateless::example()?;
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
