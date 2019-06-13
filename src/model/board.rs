use crate::{
    model::common::{Around, AxisDirection},
    util::matrix::{Matrix, MatrixIndex, MatrixShape},
};
use serde::{Deserialize, Serialize};

pub type IntersectionIndex = MatrixIndex;
pub type RoadIndex = MatrixIndex;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Board<I, R> {
    pub intersections: Matrix<I>,
    pub horizontal_roads: Matrix<R>,
    pub vertical_roads: Matrix<R>,
}

impl<I, R> Board<I, R>
where
    I: Clone,
    R: Clone,
{
    pub fn with_shape(i: I, r: R, (m, n): MatrixShape) -> Self {
        Board {
            intersections: Matrix::with_shape(i, (m, n)),
            horizontal_roads: Matrix::with_shape(r.clone(), (m, n - 1)),
            vertical_roads: Matrix::with_shape(r, (m - 1, n)),
        }
    }
}

impl<I, R> Board<I, R> {
    pub fn shape(&self) -> MatrixShape {
        self.intersections.shape()
    }

    pub fn get_roads(&self, axis: AxisDirection) -> &Matrix<R> {
        use AxisDirection::*;
        match axis {
            Horizontal => &self.horizontal_roads,
            Vertical => &self.vertical_roads,
        }
    }

    pub fn get_roads_mut(&mut self, axis: AxisDirection) -> &mut Matrix<R> {
        use AxisDirection::*;
        match axis {
            Horizontal => &mut self.horizontal_roads,
            Vertical => &mut self.vertical_roads,
        }
    }

    pub fn get_road(&self, axis: AxisDirection, index: RoadIndex) -> Option<&R> {
        self.get_roads(axis).get(index)
    }

    pub fn get_road_mut(&mut self, axis: AxisDirection, index: RoadIndex) -> Option<&mut R> {
        self.get_roads_mut(axis).get_mut(index)
    }

    pub fn roads(&self) -> impl Iterator<Item = (AxisDirection, &R)> {
        use AxisDirection::*;
        std::iter::repeat(Horizontal)
            .zip(self.horizontal_roads.iter())
            .chain(std::iter::repeat(Vertical).zip(self.vertical_roads.iter()))
    }

    pub fn roads_mut(&mut self) -> impl Iterator<Item = (AxisDirection, &mut R)> {
        use AxisDirection::*;
        std::iter::repeat(Horizontal)
            .zip(self.horizontal_roads.iter_mut())
            .chain(std::iter::repeat(Vertical).zip(self.vertical_roads.iter_mut()))
    }

    pub fn enumerate_roads(&self) -> impl Iterator<Item = (RoadIndex, (AxisDirection, &R))> {
        use AxisDirection::*;
        self.horizontal_roads
            .indices()
            .zip(std::iter::repeat(Horizontal).zip(self.horizontal_roads.iter()))
            .chain(
                self.vertical_roads
                    .indices()
                    .zip(std::iter::repeat(Vertical).zip(self.vertical_roads.iter())),
            )
    }

    // pub fn random_intersection(&self) -> IntersectionIndex {
    //     let mut rng = rand::thread_rng();
    //     let (m, n) = self.shape();
    //     (rng.gen_range(0, m), rng.gen_range(0, n));
    //     unimplemented!()
    // }

    // pub fn random_road(&self) -> (AxisDirection, RoadIndex) {
    //     let mut rng = rand::thread_rng();
    //     let direction: AxisDirection = rng.gen();
    //      self.get_roads(direction).shape()
    // }

    // pub fn random_road(&self) -> (AxisDirection, ) {
    //     let mut rng = rand::thread_rng();
    //     let direction: AxisDirection = rng.gen();
    //      self.get_roads(direction).shape()
    // }
}

pub type IntersectionContext = Around<Option<RoadIndex>>;

impl IntersectionContext {
    pub fn road_number(&self) -> usize {
        let count = |o: Option<_>| o.is_some() as usize;
        count(self.north) + count(self.south) + count(self.east) + count(self.west)
    }
}

impl<I, R> Board<I, Option<R>> {
    pub fn context_of_intersection(&self, (i, j): MatrixIndex) -> IntersectionContext {
        use AxisDirection::*;
        let north_index = if i != 0 {
            Some((Vertical, (i - 1, j)))
        } else {
            None
        };
        let south_index = Some((Vertical, (i, j)));
        let west_index = if j != 0 {
            Some((Horizontal, (i, j - 1)))
        } else {
            None
        };
        let east_index = Some((Horizontal, (i, j)));

        let check_and_convert = |o| {
            let (axis, index) = o?;
            match self.get_road(axis, index) {
                Some(option) => match option {
                    Some(_) => Some(index),
                    None => None,
                },
                None => None,
            }
        };

        IntersectionContext {
            north: check_and_convert(north_index),
            south: check_and_convert(south_index),
            east: check_and_convert(east_index),
            west: check_and_convert(west_index),
        }
    }
}
