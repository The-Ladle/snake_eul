use core::ops::Mul;
use ternary_rs::if_else;
use ternary_rs::ternary;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    data: [[f32; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub const fn new(data: [[f32; N]; M]) -> Self {
        Self { data }
    }

    pub const fn zero() -> Self {
        Self { data: [[0.0; N]; M] }
    }

    pub fn identity() -> Self {
        let min = ternary!(N < M, N, M);
        let mut data = [[0.0; N]; M];

        for i in 0..min {
            data[i][i] = 1.0;
        }

        Self { data }
    }

    pub fn transpose(&self) -> Matrix<N, M> {
        let mut data = [[0.0; M]; N];
        for i in 0..M {
            for j in 0..N {
                data[j][i] = self.data[i][j];
            }
        }
        Matrix::new(data)
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn trace(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.data[i][i];
        }
        sum
    }

    pub fn determinant(&self) -> f32 {
       todo!()
    }
}

impl<const M: usize, const N: usize, const P: usize> Mul<Matrix<N, P>> for Matrix<M, N> {
    type Output = Matrix<M, P>;

    fn mul(self, rhs: Matrix<N, P>) -> Self::Output {
        let mut data = [[0.0; P]; M];

        for m in 0..M {
            for p in 0..P {
                for n in 0..N {
                    data[m][p] += self.data[m][n] * rhs.data[n][p];
                }
            }
        }

        Matrix { data }
    }
}

impl<const M: usize, const N: usize> Mul<Vector<N>> for Matrix<M, N> {
    type Output = Vector<M>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut data = [0.0; M];

        for m in 0..M {
            for n in 0..N {
                data[m] += self.data[m][n] * rhs.data[n];
            }
        }

        Vector { data }
    }
}

impl<const M: usize, const N: usize> Mul<f32> for Matrix<M, N> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut data = self.data;
        for i in 0..M {
            for j in 0..N {
                data[i][j] *= rhs;
            }
        }
        Self { data }
    }
}