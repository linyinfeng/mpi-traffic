use crate::model::{
    generate::{
        stateful::generate_from_stateless,
        stateless::{generate_stateless_model, StatelessModelGenerationSettings},
    },
    Model,
};
use structopt::StructOpt;

pub mod stateful;
pub mod stateless;

#[derive(StructOpt)]
pub struct ModelGenerationSettings {
    #[structopt(flatten)]
    pub stateless_model_settings: StatelessModelGenerationSettings,
}

pub fn generate_model(model_settings: ModelGenerationSettings) -> Model {
    let stateless_model = generate_stateless_model(model_settings.stateless_model_settings);
    let stateful_model = generate_from_stateless(&stateless_model);
    Model {
        stateless: stateless_model,
        stateful: stateful_model,
    }
}
