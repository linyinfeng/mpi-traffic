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

pub struct Iter<'a, T: 'a> {
    matrix: &'a Matrix<T>,
    index: Indices,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    //  &'a  T    &'b T
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index.next() {
            self.matrix.get(index)
        } else {
            None
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    matrix: *mut Matrix<T>,
    index: Indices,
    _maker: std::marker::PhantomData<&'a mut Matrix<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index.next() {
            unsafe {
                (*self.matrix).get_mut(index)
            }
        } else {
            None
        }
    }
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
    pub fn iter(&self) -> Iter<T> {
        Iter {
            matrix: self,
            index: self.indices(),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            matrix: unsafe { self as *mut Matrix<T> },
            index: self.indices(),
            _maker: std::marker::PhantomData,
        }
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

    #[test]
    fn test_iter() {
        let shape = (3, 3);
        let test_mat = Matrix::with_shape(3.14, shape);
        for te in test_mat.iter() {
            assert_eq!(3.14, *te)
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut test_mat = Matrix::with_shape(3.14, (3, 3));
        for da in test_mat.iter_mut() {
            *da = 4.14;
        }
        test_mat.iter_mut().for_each(|x| assert_eq!(4.14, *x));
    }

    #[test]
    fn test_iter_mut_compile() {
        let mut test_mat = Matrix::with_shape(3.14, (3, 3));
        let mut a = test_mat.iter_mut();
        let mut b = test_mat.iter_mut();
        let r1 = a.next().unwrap();
        let r2 = b.next().unwrap();
        *r1 = 3.1;
        *r2 = 3.0;
        assert_eq!(*r1, 3.0);
    }
}
