//! Module `stateless` is the static part of the simulation

pub mod car;
pub mod intersection;
pub mod road;

use crate::model::{
    board::{Board, IntersectionIndex, RoadIndex},
    common::{AxisDirection, Geometry, Position},
};
pub use car::Car;
pub use intersection::Intersection;
pub use road::{Lane, Road};

#[derive(Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
    pub lane_width: f64,
    pub horizontal_road_length: Vec<f64>,
    pub vertical_road_length: Vec<f64>,
    pub intersection_height: Vec<f64>,
    pub intersection_width: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub city: City,
    pub cars: Vec<Car>,
}

impl City {
    pub fn geometry(&self) -> Geometry {
        let width = self
            .horizontal_road_length
            .iter()
            .chain(self.intersection_width.iter())
            .sum();
        let height = self
            .vertical_road_length
            .iter()
            .chain(self.intersection_height.iter())
            .sum();
        Geometry { width, height }
    }

    pub fn intersection_center(&self, (i, j): IntersectionIndex) -> Position {
        let x = self.intersection_width.iter().take(j).sum::<f64>() +
            self.horizontal_road_length.iter().take(j).sum::<f64>() +
            self.intersection_width[j] / 2.0;
        let y = self.intersection_height.iter().take(i).sum::<f64>() +
            self.vertical_road_length.iter().take(i).sum::<f64>() +
            self.intersection_height[i] / 2.0;
        Position { x, y }
    }

    pub fn horizontal_road_center(&self, (i, j): RoadIndex) -> Position {
        let x = self.intersection_width.iter().take(j + 1).sum::<f64>() +
            self.horizontal_road_length.iter().take(j).sum::<f64>() +
            self.horizontal_road_length[j] / 2.0;
        let y = self.intersection_height.iter().take(i).sum::<f64>() +
            self.vertical_road_length.iter().take(i).sum::<f64>() +
            self.intersection_height[i] / 2.0;
        Position { x, y }
    }

    pub fn vertical_road_center(&self, (i, j): RoadIndex) -> Position {
        let x = self.intersection_width.iter().take(j).sum::<f64>() +
            self.horizontal_road_length.iter().take(j).sum::<f64>() +
            self.intersection_width[j] / 2.0;
        let y = self.intersection_height.iter().take(i + 1).sum::<f64>() +
            self.vertical_road_length.iter().take(i).sum::<f64>() +
            self.vertical_road_length[i] / 2.0;
        Position { x, y }
    }

    pub fn road_center(&self, direction: AxisDirection, index: RoadIndex) -> Position {
        use AxisDirection::*;
        match direction {
            Horizontal => self.horizontal_road_center(index),
            Vertical => self.vertical_road_center(index),
        }
    }

    pub fn road_length(&self, direction: AxisDirection, (i, j): RoadIndex) -> f64 {
        use AxisDirection::*;
        match direction {
            Horizontal => self.horizontal_road_length[j],
            Vertical => self.vertical_road_length[i],
        }
    }

    pub fn intersection_geometry(&self, (i, j): IntersectionIndex) -> Geometry {
        Geometry {
            width: self.intersection_width[j],
            height: self.intersection_height[i],
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn example_city() -> City {
        City {
            board: Board::with_shape(None, None, (3, 3)),
            lane_width: 3.5,
            horizontal_road_length: vec![500.0, 500.0],
            vertical_road_length: vec![500.0, 500.0],
            intersection_height: vec![20.0, 20.0, 20.0],
            intersection_width: vec![20.0, 20.0, 20.0],
        }
    }

    #[test]
    fn intersection_center() {
        let city = example_city();
        let answer = [
            [(10.0, 10.0), (530.0, 10.0), (1050.0, 10.0)],
            [(10.0, 530.0), (530.0, 530.0), (1050.0, 530.0)],
            [(10.0, 1050.0), (530.0, 1050.0), (1050.0, 1050.0)],
        ];
        for (i, j) in city.board.intersections.indices() {
            let Position { x, y } = city.intersection_center((i, j));
            assert_eq!((x, y), answer[i][j], "position: {:?}", (i, j));
        }
    }

    #[test]
    fn geometry() {
        let city = example_city();
        let Position { x, y } = city.intersection_center((2, 2));
        let width = x + city.intersection_width[2] / 2.0;
        let height = y + city.intersection_height[2] / 2.0;
        assert_eq!(Geometry { width, height }, city.geometry());
    }

    #[test]
    fn horizontal_roads_center() {
        let city = example_city();
        let answer = [
            [(270.0, 10.0), (790.0, 10.0)],
            [(270.0, 530.0), (790.0, 530.0)],
            [(270.0, 1050.0), (790.0, 1050.0)],
        ];
        for (i, j) in city.board.horizontal_roads.indices() {
            let Position { x, y } = city.horizontal_road_center((i, j));
            assert_eq!((x, y), answer[i][j], "position: {:?}", (i, j));
        }
    }

    #[test]
    fn vertical_roads_center() {
        let city = example_city();
        let answer = [
            [(10.0, 270.0), (530.0, 270.0), (1050.0, 270.0)],
            [(10.0, 790.0), (530.0, 790.0), (1050.0, 790.0)],
        ];
        for (i, j) in city.board.vertical_roads.indices() {
            let Position { x, y } = city.vertical_road_center((i, j));
            assert_eq!((x, y), answer[i][j], "position: {:?}", (i, j));
        }
    }
}
