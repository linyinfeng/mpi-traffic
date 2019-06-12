use crate::model::{stateful::Model, stateless};

pub mod city;

pub fn generate_from_stateless(stateless_model: &stateless::Model) -> Model {
    Model {
        city: city::generate_city_from_stateless(&stateless_model.city),
        cars: Vec::new(),
    }
}
