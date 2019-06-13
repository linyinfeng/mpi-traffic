//! Module `stateful` is the dynamic part of the simulation

use crate::model::board::Board;
use serde::{Deserialize, Serialize};

pub mod car;
pub mod intersection;

pub use car::Car;
pub use intersection::Intersection;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, ()>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Option<Car>>,
}
