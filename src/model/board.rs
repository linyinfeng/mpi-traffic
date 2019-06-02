use crate::matrix::Matrix;

pub struct Board<P, L> {
    pub point: Matrix<P>,
    pub horizontal: Matrix<L>,
    pub vertical: Matrix<L>,
}

impl<P, L> Board<P, L>
where
    P: Clone,
    L: Clone,
{
    pub fn with_size(p: P, l: L, (m, n): (usize, usize)) -> Self {
        Board {
            point: Matrix::with_size(p, (m, n)),
            horizontal: Matrix::with_size(l.clone(), (m, n - 1)),
            vertical: Matrix::with_size(l, (m - 1, n)),
        }
    }
}
