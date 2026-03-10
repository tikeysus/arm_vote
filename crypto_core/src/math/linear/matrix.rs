use crate::modint::const_modint::ConstModInt;
use crate::errors::CryptoError;

#[derive(Debug, Clone)]
pub struct Matrix<const P: u64> {
    pub data: Vec<Vec<ConstModInt<P>>>,
}

impl<const P: u64> Matrix<P> {

    pub fn new(data: Vec<Vec<ConstModInt<P>>>) -> Self {
        Self { data }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        if self.rows() == 0 {
            0
        } else {
            self.data[0].len()
        }
    }

    // ---------------------------
    // MATRIX ADDITION
    // ---------------------------
    pub fn add(&self, other: &Self) -> Result<Self, CryptoError> {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return Err(CryptoError::MatrixDimensionMismatch);
        }

        let mut result = vec![];

        for i in 0..self.rows() {
            let mut row = vec![];

            for j in 0..self.cols() {
                row.push(self.data[i][j].add(other.data[i][j]));
            }

            result.push(row);
        }

        Ok(Matrix::new(result))
    }

    // ---------------------------
    // MATRIX MULTIPLICATION
    // ---------------------------
    pub fn mul(&self, other: &Self) -> Result<Self, CryptoError> {

        if self.cols() != other.rows() {
            return Err(CryptoError::MatrixDimensionMismatch);
        }

        let mut result = vec![
            vec![ConstModInt::<P>::new(0)?; other.cols()];
            self.rows()
        ];

        for i in 0..self.rows() {
            for j in 0..other.cols() {

                let mut sum = ConstModInt::<P>::new(0)?;

                for k in 0..self.cols() {
                    let product = self.data[i][k].mul(other.data[k][j]);
                    sum = sum.add(product);
                }

                result[i][j] = sum;
            }
        }

        Ok(Matrix::new(result))
    }

    // ---------------------------
    // MATRIX-VECTOR MULTIPLICATION
    // ---------------------------
    pub fn mul_vector(
        &self,
        vector: &Vec<ConstModInt<P>>,
    ) -> Result<Vec<ConstModInt<P>>, CryptoError> {

        if self.cols() != vector.len() {
            return Err(CryptoError::MatrixDimensionMismatch);
        }

        let mut result = vec![];

        for i in 0..self.rows() {

            let mut sum = ConstModInt::<P>::new(0)?;

            for j in 0..self.cols() {
                let product = self.data[i][j].mul(vector[j]);
                sum = sum.add(product);
            }

            result.push(sum);
        }

        Ok(result)
    }
}