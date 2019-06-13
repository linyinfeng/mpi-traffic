//! Module `stateless` is the static part of the simulation

pub mod car;
pub mod intersection;
pub mod road;

use crate::model::{
    board::{Board, IntersectionIndex, RoadIndex},
    common::{
        AbsoluteDirection, AxisDirection, Geometry, InOutDirection, LaneDirection, LaneIndex,
        Position,
    },
};
pub use car::Car;
pub use intersection::Intersection;
pub use road::{Lane, Road};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct City {
    pub board: Board<Option<Intersection>, Option<Road>>,
    pub car_out_intersection: IntersectionIndex,
    pub car_out_min_distance: f64,
    pub lane_width: f64,
    pub horizontal_road_length: Vec<f64>,
    pub vertical_road_length: Vec<f64>,
    pub intersection_height: Vec<f64>,
    pub intersection_width: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    /// Return the join point relative to intersection center
    pub fn intersection_road_join_position(
        &self,
        intersection_index: IntersectionIndex,
        direction: AbsoluteDirection,
        in_out: InOutDirection,
        lane_index: LaneIndex,
    ) -> Option<Position> {
        use AbsoluteDirection::*;
        let geometry = self.intersection_geometry(intersection_index);
        let context = self.board.context_of_intersection(intersection_index);
        let road_index = (*context.get(direction))?;
        let road_direction = direction.axis_direction();
        let lane_direction = LaneDirection::absolute_in_out_to_lane(direction, in_out);
        let road = self.board.get_road(road_direction, road_index)?.as_ref()?;
        let offset = self.lane_center_offset(road, lane_direction, lane_index);
        let position = match direction {
            North => Position {
                x: -offset,
                y: geometry.height / 2.0,
            },
            South => Position {
                x: -offset,
                y: -geometry.height / 2.0,
            },
            East => Position {
                x: geometry.height / 2.0,
                y: offset,
            },
            West => Position {
                x: -geometry.height / 2.0,
                y: offset,
            },
        };
        Some(position)
    }

    /// Return the join point relative to intersection center
    pub fn intersection_path_total_length(
        &self,
        intersection_index: IntersectionIndex,
        from_direction: AbsoluteDirection,
        from_lane_index: LaneIndex,
        to_direction: AbsoluteDirection,
        to_lane_index: LaneIndex,
    ) -> Option<f64> {
        let from_position = self.intersection_road_join_position(
            intersection_index,
            from_direction,
            InOutDirection::In,
            from_lane_index,
        )?;
        let to_position = self.intersection_road_join_position(
            intersection_index,
            to_direction,
            InOutDirection::Out,
            to_lane_index,
        )?;
        Some(from_position.distance(to_position))
    }

    pub fn lane_center_offset(
        &self,
        road: &Road,
        direction: LaneDirection,
        lane_index: usize,
    ) -> f64 {
        let lane_number = road.lane_number();
        let top = -self.lane_width * lane_number as f64 / 2.0;
        let lane_offset = match direction {
            LaneDirection::HighToLow => road.lane_to_low.len() - 1 - lane_index,
            LaneDirection::LowToHigh => road.lane_to_low.len() + lane_index,
        };
        top + lane_offset as f64 * self.lane_width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_city() -> City {
        City {
            board: Board::with_shape(None, None, (3, 3)),
            car_out_intersection: (0, 0),
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
