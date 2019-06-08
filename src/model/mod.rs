pub mod board;
pub mod common;
pub mod generate;
pub mod stateful;
pub mod stateless;

pub struct Model {
    pub stateless: stateless::Model,
    pub stateful: stateful::Model,
}
