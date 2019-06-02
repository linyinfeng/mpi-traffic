pub mod board;
pub mod intersection;
pub mod road;

use board::Board;
use intersection::Intersection;
use road::Road;

pub struct City {
    board: Board<Intersection, Road>,
    horizontal_road_length: Vec<f64>,
    vertical_road_length: Vec<f64>,
}
