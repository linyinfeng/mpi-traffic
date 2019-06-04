use std::iter::ExactSizeIterator;
use std::iter::FusedIterator;
use std::iter::Iterator;
use std::ops::Index;
use std::ops::IndexMut;

pub type MatrixShape = (usize, usize);
pub type MatrixIndex = (usize, usize);

#[derive(Clone, Debug, Default)]
pub struct Matrix<T> {
    storage: Vec<Vec<T>>,
}

impl<T> Index<MatrixIndex> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): MatrixIndex) -> &Self::Output {
        &self.storage[i][j]
    }
}

impl<T> IndexMut<MatrixIndex> for Matrix<T> {
    fn index_mut(&mut self, (i, j): MatrixIndex) -> &mut Self::Output {
        &mut self.storage[i][j]
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, (i, j): MatrixIndex) -> Option<&T> {
        self.storage.get(i).and_then(|row| row.get(j))
    }

    pub fn get_mut(&mut self, (i, j): MatrixIndex) -> Option<&mut T> {
        self.storage.get_mut(i).and_then(|row| row.get_mut(j))
    }

    pub fn shape(&self) -> MatrixShape {
        let col_length = self.storage.get(0).map_or(0, |row| row.len());
        (self.storage.len(), col_length)
    }

    pub fn indices(&self) -> Indices {
        Indices {
            shape: self.shape(),
            index: (0, 0),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.storage.iter().flat_map(|inner| inner.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.storage.iter_mut().flat_map(|inner| inner.iter_mut())
    }

    pub fn enumerate(&self) -> impl Iterator<Item=((usize, usize), &T)> {
        self.indices().zip(self.iter())
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item=((usize, usize), &mut T)> {
        self.indices().zip(self.iter_mut())
    }
}

impl<T> Matrix<T>
    where
        T: Clone,
{
    pub fn with_shape(t: T, (i, j): MatrixShape) -> Self {
        Matrix {
            storage: vec![vec![t; j]; i],
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
