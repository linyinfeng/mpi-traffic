//! Module `stateless` is the static part of the simulation

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
    pub lane_width: f64,
    pub horizontal_road_length: Vec<f64>,
    pub vertical_road_length: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Car>,
}
