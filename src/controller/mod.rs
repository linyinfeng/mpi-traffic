use crate::model::stateful;
use crate::model::stateless;
use piston_window::Input;
use piston_window::UpdateArgs;

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
