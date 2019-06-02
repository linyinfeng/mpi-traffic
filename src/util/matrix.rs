use std::ops::Index;
use std::ops::IndexMut;

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

    pub fn shape(&self) -> MatrixIndex {
        let col_length = self.storage.get(0).map_or(0, |row| row.len());
        (self.storage.len(), col_length)
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn with_shape(t: T, (i, j): MatrixIndex) -> Self {
        Matrix {
            storage: vec![vec![t; j]; i],
        }
    }
}

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
}
