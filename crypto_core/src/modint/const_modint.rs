use crate::errors::CryptoError;
use crate::math::number_theory::mod_inverse::modular_inverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstModInt<const P: u64> {
    pub value: u64,
}

impl<const P: u64> ConstModInt<P> {
    pub fn new(x: u64) -> Result<Self, CryptoError> {
        if P == 0 {
            return Err(CryptoError::ModulusIsZero);
        }
        Ok(Self { value: x % P })
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn add(self, other: Self) -> Result<Self, CryptoError> {
        let result = ((self.value as u128 + other.value as u128) % P as u128) as u64;
        Self::new(result)
    }

    pub fn sub(self, other: Self) -> Result<Self, CryptoError> {
        let result = ((self.value as u128 + P as u128 - other.value as u128) % P as u128) as u64;
        Self::new(result)
    }

    pub fn mul(self, other: Self) -> Result<Self, CryptoError> {
        let result = ((self.value as u128 * other.value as u128) % P as u128) as u64;
        Self::new(result)
    }

    pub fn pow(self, mut exponent: u64) -> Result<Self, CryptoError> {
        if exponent == 0 {
            return Self::new(1);
        }
        if exponent == 1 {
            return Ok(self);
        }

        let mut result = 1u128;
        let mut base = self.value as u128 % P as u128;
        while exponent > 0 {
            if exponent & 1 == 1 {
                result = (result * base) % P as u128;
            }
            base = (base * base) % P as u128;
            exponent >>= 1;
        }
        Self::new(result as u64)
    }

    pub fn inverse(self) -> Result<Self, CryptoError> {
        modular_inverse(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_handles_values_that_overflow_u64_before_reduction() {
        type F = ConstModInt<{ u64::MAX }>;

        let left = F::new(u64::MAX - 1).unwrap();
        let right = F::new(u64::MAX - 2).unwrap();

        assert_eq!(left.add(right).unwrap().value(), u64::MAX - 3);
    }

    #[test]
    fn mul_handles_values_that_overflow_u64_before_reduction() {
        const P: u64 = u64::MAX - 58;
        type F = ConstModInt<P>;

        let left = F::new(P - 2).unwrap();
        let right = F::new(P - 3).unwrap();
        let expected = ((P - 2) as u128 * (P - 3) as u128 % P as u128) as u64;

        assert_eq!(left.mul(right).unwrap().value(), expected);
    }
}
