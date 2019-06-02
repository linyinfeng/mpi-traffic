pub mod board;
pub mod common;
pub mod intersection;
pub mod road;
pub mod car;

use board::Board;
use intersection::Intersection;
use road::Road;

pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
    pub horizontal_road_length: Vec<f64>,
    pub vertical_road_length: Vec<f64>,
}
