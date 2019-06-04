use crate::model::common::Around;
use crate::model::common::HorizontalOrVertical;
use crate::util::matrix::Matrix;
use crate::util::matrix::MatrixIndex;
use crate::util::matrix::MatrixShape;

pub type IntersectionIndex = MatrixIndex;
pub type RoadIndex = MatrixIndex;

#[derive(Clone, Debug)]
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
    pub fn get_road(&self, h_or_v: HorizontalOrVertical, index: RoadIndex) -> Option<&R> {
        use HorizontalOrVertical::*;
        match h_or_v {
            Horizontal => self.horizontal_roads.get(index),
            Vertical => self.vertical_roads.get(index),
        }
    }

    pub fn get_road_mut(
        &mut self,
        h_or_v: HorizontalOrVertical,
        index: RoadIndex,
    ) -> Option<&mut R> {
        use HorizontalOrVertical::*;
        match h_or_v {
            Horizontal => self.horizontal_roads.get_mut(index),
            Vertical => self.vertical_roads.get_mut(index),
        }
    }
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
        use HorizontalOrVertical::*;
        let north_index = if i != 0 {
            Some((Vertical, (i - 1, j)))
        } else {
            None
        };
        let south_index = Some((Vertical, (i, j)));
        let east_index = if j != 0 {
            Some((Horizontal, (i, j - 1)))
        } else {
            None
        };
        let west_index = Some((Horizontal, (i, j)));

        let check_and_convert = |o| {
            let (h_or_v, index) = o?;
            match self.get_road(h_or_v, index) {
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
