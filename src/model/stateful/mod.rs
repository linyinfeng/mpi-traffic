//! Module `stateful` is the dynamic part of the simulation

pub mod car;
pub mod intersection;
pub mod road;

pub use car::Car;
pub use intersection::Intersection;
pub use road::Lane;
pub use road::Road;

use crate::model::board::Board;

#[derive(Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Car>,
}
