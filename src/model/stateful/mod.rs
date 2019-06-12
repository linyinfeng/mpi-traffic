//! Module `stateful` is the dynamic part of the simulation

pub mod car;
pub mod intersection;
pub mod road;

pub use car::Car;
pub use intersection::Intersection;
pub use road::{Lane, Road};

use crate::model::board::Board;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Option<Car>>,
}
