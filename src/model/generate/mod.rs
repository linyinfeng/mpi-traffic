use crate::model::stateful as m_stateful;
use crate::model::stateless as m_stateless;

pub mod stateful;
pub mod stateless;

pub fn example() -> Result<(m_stateless::Model, m_stateful::Model), crate::Error> {
    let stateless = stateless::example()?;
    let stateful = stateful::generate_from_stateless(&stateless);
    Ok((stateless, stateful))
}
