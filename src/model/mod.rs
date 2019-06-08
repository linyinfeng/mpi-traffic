pub mod board;
pub mod common;
pub mod generate;
pub mod stateful;
pub mod stateless;

pub struct Model {
    stateless: stateless::Model,
    stateful: stateful::Model,
}
