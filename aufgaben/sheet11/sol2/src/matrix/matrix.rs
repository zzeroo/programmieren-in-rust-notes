/// http://jadpole.github.io/rust/typechecked-matrix
use std::num::{One, Zero};

use integer_generics::*;

use ::std::marker::PhantomData;
use ::std::ops::{Add, Mul};
use ::std::fmt;


// #[derive(Copy, PartialEq)]
#[derive(PartialEq)]
pub struct Matrix<M: Num, N: Num> {
    rows: PhantomData<M>,
    cols: PhantomData<N>,
    entries: Vec<Vec<f64>>,
}

impl<M: Num, N: Num> Matrix<M, N> {
    pub fn new_map<F>(func: F) -> Matrix<M, N>
        where   F: Fn(usize, usize) -> f64,
    {
        Matrix {
            rows: PhantomData,
            cols: PhantomData,
            entries: (0..M::val()).map(|row|
                (0..N::val()).map(|col|
                    func(row, col)
                ).collect()
            ).collect()
        }
    }
}

/// # Examples
///
/// ```rust
/// use ttt::*;
///
/// let i3 = Matrix::<N3, N3>::identity();
/// ```
///
/// ```rust
/// use ttt::*;
///
/// let m = Matrix::<N3, N2>::new_map(|i, j| (i + j) as f64);
/// let m2 = Matrix::identity() * m.clone();
///
/// assert!(m == m2);
/// ```
impl<M: Num> Matrix<M, M> {
    pub fn identity() -> Matrix<M, M> {
        Matrix::new_map(|i, j| if i == j { One::one() } else { Zero::zero() })
    }
}

impl<M: Num, N: Num> Matrix<M, N> {
    pub fn transpose(&self) -> Matrix<N, M> {
        Matrix::new_map(|i, j| self.entries[j][i])
    }
}

// Matrix multiplication

/// matrix * scalar
impl<M: Num, N: Num> Mul<f64> for Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn mul(self, rhs: f64) -> Matrix<M, N> {
        Matrix::new_map(|i, j| self.entries[i][j] * rhs)
    }
}

// scalar * matrix
impl<M: Num, N: Num> Mul<Matrix<M, N>> for f64 {
    type Output = Matrix<M, N>;
    fn mul(self, rhs: Matrix<M, N>) -> Matrix<M, N> {
        rhs * self
    }
}

impl<M: Num, N: Num, L: Num> Mul<Matrix<N, L>> for Matrix<M, N> {
    type Output = Matrix<M, L>;
    fn mul(self: Matrix<M, N>, rhs: Matrix<N, L>) -> Matrix<M, L> {
        Matrix::new_map(|i, j|
            (0..N::val()).map(|k| self.entries[i][k] * rhs.entries[k][j])
                .fold(0.0, Add::add))
    }
}

// /// If Iterator::sum stabilizes, we will be able to something much closer to the mathematical definition:
// ///
// impl<M: Num, N: Num, L: Num> Mul<Matrix<M, L>> for Matrix<M, N> {
//     type Output = Matrix<M, L>;
//     fn mul(self: Matrix<M, N>, rhs: Matrix<N, L>) -> Matrix<M, L> {
//         Matrix::new_map(|i, j|
//             (0..N::val()).map(|k| self.entries[i][k] * rhs.entries[k][j])).sum()
//     }
// }


// Vector Support
// type Vector<N> = Matrix<N, N1>;
// type Covector<N> = Matrix<N1, N>;

type Ket<N> = Matrix<N, N1>;
type Bra<N> = Matrix<N1, N>;

impl<N: Num> Ket<N> {
    fn vec_entries(&self) -> Vec<f64> {
        self.entries.iter().map(|row| row[0]).collect()
    }
}

// impl<N: Num> Bra<N> {
//     fn vec_entries(&self) -> Vec<f64> {
//         self.entries[0].clone()
//     }
// }



/// Debug trait implementation
///
impl<M: Num, N: Num> fmt::Debug for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix")
    }
}
