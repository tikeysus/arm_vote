use crate::errors::CryptoError;
use crate::modint::const_modint::ConstModInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<const P: u64> {
    pub data: Vec<ConstModInt<P>>,
}

impl<const P: u64> Vector<P> {

    pub fn new(data: Vec<ConstModInt<P>>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    // ---------------------------
    // SCALAR MULTIPLICATION
    // ---------------------------
    pub fn scalar_mul(&self, c: ConstModInt<P>) -> Self {

        let data = self
            .data
            .iter()
            .map(|x| x.mul(c))
            .collect();

        Self { data }
    }

    // ---------------------------
    // VECTOR ADDITION
    // ---------------------------
    pub fn add(&self, other: &Self) -> Result<Self, CryptoError> {

        if self.len() != other.len() {
            return Err(CryptoError::VectorDimensionMismatch);
        }

        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(left, right)| left.add(*right))
            .collect();

        Ok(Self { data })
    }

    // ---------------------------
    // DOT PRODUCT
    // ---------------------------
    pub fn dot(&self, other: &Self) -> Result<ConstModInt<P>, CryptoError> {

        if self.len() != other.len() {
            return Err(CryptoError::VectorDimensionMismatch);
        }

        let mut sum = ConstModInt::<P>::new(0)?;

        for (a, b) in self.data.iter().zip(other.data.iter()) {
            let product = a.mul(*b);
            sum = sum.add(product);
        }

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    type F = ConstModInt<13>;

    fn make_vector(values: Vec<u64>) -> Vector<13> {
        Vector::new(values.into_iter().map(|x| F::new(x).unwrap()).collect())
    }

    #[test]
    fn scalar_mul_works() {

        let v = make_vector(vec![2,3,4]);

        let result = v.scalar_mul(F::new(3).unwrap());

        assert_eq!(
            result.data.iter().map(|x| x.value()).collect::<Vec<_>>(),
            vec![6,9,12]
        );
    }

    #[test]
    fn vector_addition_works() {

        let a = make_vector(vec![1,2,3]);
        let b = make_vector(vec![10,20,30]);

        let result = a.add(&b).unwrap();

        assert_eq!(
            result.data.iter().map(|x| x.value()).collect::<Vec<_>>(),
            vec![11,22,33]
        );
    }

    #[test]
    fn dot_product_works() {

        let a = make_vector(vec![1,2,3]);
        let b = make_vector(vec![4,5,6]);

        let result = a.dot(&b).unwrap();

        assert_eq!(result.value(), 32 % 13);
    }

    #[test]
    fn dimension_mismatch() {

        let a = make_vector(vec![1,2,3]);
        let b = make_vector(vec![4,5]);

        assert_eq!(a.add(&b), Err(CryptoError::VectorDimensionMismatch));
    }
}