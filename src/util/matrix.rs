use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone, Debug, Default)]
pub struct Matrix<T> {
    storage: Vec<Vec<T>>,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.storage[i][j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.storage[i][j]
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, (i, j): (usize, usize)) -> Option<&T> {
        self.storage.get(i).and_then(|row| row.get(j))
    }

    pub fn get_mut(&mut self, (i, j): (usize, usize)) -> Option<&mut T> {
        self.storage.get_mut(i).and_then(|row| row.get_mut(j))
    }

    pub fn shape(&self) -> (usize, usize) {
        let col_length = self.storage.get(0).map_or(0, |row| row.len());
        (self.storage.len(), col_length)
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn with_shape(t: T, (i, j): (usize, usize)) -> Self {
        Matrix {
            storage: vec![vec![t; j]; i],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut mat_f64 = Matrix::with_shape(3.14, (7, 9));
        mat_f64[(3, 0)] = 4.14;
        assert_eq!((7, 9), mat_f64.shape());
        assert_eq!(4.14, mat_f64[(3, 0)]);
        assert_eq!(None, mat_f64.get((9, 9)));
    }
}
