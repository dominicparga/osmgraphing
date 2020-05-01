use crate::defaults::capacity::DimVec;
use nalgebra;
use std::{
    fmt::{self, Display},
    ops::Index,
};

#[derive(Debug)]
pub struct Matrix {
    data: nalgebra::DMatrix<f64>,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Matrix {
    pub fn from_rows(rows: DimVec<DimVec<f64>>) -> Matrix {
        let dim_0 = rows.len();
        let dim_1 = rows[0].len();
        let mut values = Vec::new();

        for row in rows {
            for value in row {
                values.push(value);
            }
        }

        Matrix {
            data: nalgebra::DMatrix::from_row_slice(dim_0, dim_1, &values),
        }
    }

    pub fn lu(self) -> LU {
        LU {
            data: self.data.lu(),
        }
    }
}

impl Index<usize> for Matrix {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.data[i]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        let (_n_rows, n_cols) = self.data.shape();
        &self.data[n_cols * col + row]
    }
}

pub struct LU {
    data: nalgebra::LU<f64, nalgebra::Dynamic, nalgebra::Dynamic>,
}

impl LU {
    pub fn solve(&self, b: &DimVec<f64>) -> Option<DimVec<f64>> {
        let b = nalgebra::DVector::from_row_slice(&b);
        let x = self.data.solve(&b)?;
        Some(DimVec::from_slice(x.data.as_vec()))
    }
}
