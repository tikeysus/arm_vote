use crate::errors::CryptoError;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Vector{
    data: Vec<u64>,
}

impl Vector {
    pub fn scalar_mult_in_place(&mut self, c: u64) -> Result<(), CryptoError> {
        for x in &mut self.data {
            *x = x.checked_mul(c).ok_or(CryptoError::Overflow)?;
        }
        Ok(())
    }
    pub fn scalar_mult(mut self, c: u64) -> Result<Self, CryptoError> {
        self.scalar_mult_in_place(c)?;
        Ok(self)
    }

    pub fn vector_addition(self, other: Self) -> Result<Self, CryptoError>{
        if self.data.len() != other.data.len() {
            return Err(CryptoError::DifferingLengths); 
        }

        let data = self
            .data
            .into_iter()
            .zip(other.data)
            .map(|(left, right)| left.checked_add(right).ok_or(CryptoError::Overflow))
            .collect::<Result<Vec<u64>, CryptoError>>()?;

        Ok(Self { data })
    }

    pub fn dot_product(&self, other: &Self) -> Result<u64, CryptoError> {
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vector(data: Vec<u64>) -> Vector {
        Vector { data }
    }

    #[test]
    fn scalar_mult_multiplies_each_element() {
        let vector = make_vector(vec![2, 3, 4]);

        let result = vector.scalar_mult(3).expect("scalar multiplication should succeed");

        assert_eq!(result.data, vec![6, 9, 12]);
    }

    #[test]
    fn scalar_mult_returns_overflow_error() {
        let vector = make_vector(vec![u64::MAX]);

        let result = vector.scalar_mult(2);

        assert_eq!(result, Err(CryptoError::Overflow));
    }

    #[test]
    fn vector_addition_adds_matching_vectors() {
        let left = make_vector(vec![1, 2, 3]);
        let right = make_vector(vec![10, 20, 30]);

        let result = left
            .vector_addition(right)
            .expect("vector addition should succeed");

        assert_eq!(result.data, vec![11, 22, 33]);
    }

    #[test]
    fn vector_addition_returns_differing_lengths_error() {
        let left = make_vector(vec![1, 2, 3]);
        let right = make_vector(vec![10, 20]);

        let result = left.vector_addition(right);

        assert_eq!(result, Err(CryptoError::DifferingLengths));
    }
}