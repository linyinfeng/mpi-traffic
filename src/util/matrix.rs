use serde::{Deserialize, Serialize};
use std::{
    iter::{ExactSizeIterator, FusedIterator},
    ops::{Index, IndexMut},
};

pub type MatrixShape = (usize, usize);
pub type MatrixIndex = (usize, usize);

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Matrix<T> {
    pub shape: MatrixShape,
    pub storage: Vec<T>,
}

impl<T> Index<MatrixIndex> for Matrix<T> {
    type Output = T;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.storage[self.offset(index).unwrap()]
    }
}

impl<T> IndexMut<MatrixIndex> for Matrix<T> {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        let offset = self.offset(index).unwrap();
        &mut self.storage[offset]
    }
}

impl<T> Matrix<T> {
    pub fn offset(&self, (i, j): MatrixIndex) -> Option<usize> {
        let (m, n) = self.shape;
        if i < m && j < n {
            Some(i * n + j)
        } else {
            None
        }
    }

    pub fn offset_unchecked(&self, (i, j): MatrixIndex) -> usize {
        let (_m, n) = self.shape;
        i * n + j
    }

    pub fn index_from_offset(&self, offset: usize) -> Option<MatrixIndex> {
        if offset < self.storage.len() {
            Some(self.index_from_offset_unchecked(offset))
        } else {
            None
        }
    }

    pub fn index_from_offset_unchecked(&self, offset: usize) -> MatrixIndex {
        let (_m, n) = self.shape;
        (offset / n, offset % n)
    }

    pub fn shape(&self) -> MatrixShape {
        self.shape
    }

    pub fn get(&self, index: MatrixIndex) -> Option<&T> {
        self.storage.get(self.offset(index)?)
    }

    pub fn get_mut(&mut self, index: MatrixIndex) -> Option<&mut T> {
        let offset = self.offset(index)?;
        self.storage.get_mut(offset)
    }

    pub fn indices(&self) -> Indices {
        Indices {
            shape: self.shape,
            index: (0, 0),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.storage.iter_mut()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (MatrixIndex, &T)> {
        self.indices().zip(self.iter())
    }

    #[allow(clippy::type_complexity)]
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut T)> {
        self.indices().zip(self.iter_mut())
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn with_shape(t: T, (m, n): MatrixShape) -> Self {
        Matrix {
            shape: (m, n),
            storage: vec![t; m * n],
        }
    }
}

pub struct Indices {
    shape: MatrixShape,
    index: MatrixIndex,
}

impl Iterator for Indices {
    type Item = MatrixIndex;

    fn next(&mut self) -> Option<Self::Item> {
        let (m, n) = self.shape;
        let (i, j) = self.index;
        let item = if i < m && j < n {
            Some(self.index)
        } else {
            None
        };
        if item.is_some() {
            if j + 1 == n {
                // should increase row index
                self.index.0 += 1; // out of range will cause the iterator to be fused
                self.index.1 = 0;
            } else {
                // increase column index
                self.index.1 += 1;
            }
        }

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (m, n) = self.shape;
        let size = m * n;
        (size, Some(size))
    }
}

impl FusedIterator for Indices {}
impl ExactSizeIterator for Indices {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_offset() {
        let m = Matrix::with_shape(3.14, (7, 9));
        let mut n = 0;
        for i in 0..7 {
            for j in 0..9 {
                assert_eq!(m.offset((i, j)), Some(n));
                assert_eq!(m.offset_unchecked((i, j)), n);
                assert_eq!(m.index_from_offset(n), Some((i, j)));
                assert_eq!(m.index_from_offset_unchecked(n), (i, j));
                n += 1;
            }
        }
        assert_eq!(m.offset((7, 3)), None);
        assert_eq!(m.offset((3, 9)), None);
        assert_eq!(m.index_from_offset(64), None);
    }

    #[test]
    fn empty_matrix() {
        let m = Matrix::with_shape(3.14, (0, 0));
        assert_eq!(m.shape(), (0, 0));
    }

    #[test]
    fn normal_matrix() {
        let mut m = Matrix::with_shape(3.14, (7, 9));
        m[(3, 0)] = 4.14;
        for i in 0..7 {
            for j in 0..9 {
                assert_eq!(
                    m.get((i, j)).cloned(),
                    Some(if (i, j) == (3, 0) { 4.14 } else { 3.14 })
                );
                assert_eq!(
                    m.get_mut((i, j)).cloned(),
                    Some(if (i, j) == (3, 0) { 4.14 } else { 3.14 })
                );
            }
        }

        assert_eq!((7, 9), m.shape());
        assert_eq!(None, m.get((9, 9)));
    }

    #[test]
    #[should_panic]
    fn panic_out_of_m_range() {
        let mut m = Matrix::with_shape(3.14, (7, 9));
        m[(10, 3)] = 4.14;
    }

    #[test]
    #[should_panic]
    fn panic_out_of_n_range() {
        let mut m = Matrix::with_shape(3.14, (7, 9));
        m[(0, 11)] = 4.14;
    }

    #[test]
    fn matrix_indices() {
        let shape = (7, 9);
        let (n, m) = shape;
        let mat = Matrix::with_shape(3.14, shape);
        let mut indices = mat.indices();
        for i in 0..n {
            for j in 0..m {
                assert_eq!(indices.next().unwrap(), (i, j));
            }
        }
    }

    #[test]
    fn enumerate_mut() {
        let shape = (7, 9);
        let (m, n) = shape;
        let mut mat = Matrix::with_shape(3.14, shape);
        let v = |i, j| i as f64 * 10.0 + j as f64;
        for ((i, j), item) in mat.enumerate_mut() {
            *item = v(i, j);
        }
        for i in 0..m {
            for j in 0..n {
                assert_eq!(mat[(i, j)], v(i, j));
            }
        }
    }

    #[test]
    fn enumerate() {
        let mut mat = Matrix::with_shape(3.14, (7, 9));
        let v = |i, j| i as f64 * 10.0 + j as f64;
        for ((i, j), item) in mat.enumerate_mut() {
            *item = v(i, j);
        }
        for ((i, j), item) in mat.enumerate() {
            assert_eq!(*item, v(i, j));
        }
    }
}
