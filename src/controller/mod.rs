use crate::model::{stateful, stateless};
use piston_window::{Input, UpdateArgs};

pub mod car_map;

#[derive(Clone, Debug)]
pub struct Controller;

impl Controller {
    pub fn update(
        &self,
        _stateful: &stateless::Model,
        _stateless: &mut stateful::Model,
        _args: UpdateArgs,
    ) {
    }

    pub fn input(
        &self,
        _stateful: &stateless::Model,
        _stateless: &mut stateful::Model,
        _input: Input,
    ) {
    }
}
