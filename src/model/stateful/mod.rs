//! Module `stateful` is the dynamic part of the simulation

pub mod car;
pub mod intersection;
pub mod road;

use crate::model::board::Board;
use car::Car;
use intersection::Intersection;
use road::Road;

#[derive(Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Car>,
}
