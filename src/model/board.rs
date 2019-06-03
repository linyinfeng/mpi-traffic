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
