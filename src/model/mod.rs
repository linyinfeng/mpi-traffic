pub mod board;
pub mod common;
pub mod stateful;
pub mod stateless;

#[derive(Clone, Debug)]
pub struct Model {
    pub stateless: stateless::Model,
    pub stateful: stateful::Model,
}
