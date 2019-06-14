use crate::model::{stateful::Model, stateless};

pub mod city;

pub fn generate_from_stateless(stateless_model: &stateless::Model) -> Model {
    let car_number = stateless_model.cars.len();
    Model {
        city: city::generate_city_from_stateless(&stateless_model.city),
        cars: vec![None; car_number],
    }
}
