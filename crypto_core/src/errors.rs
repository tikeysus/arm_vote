#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    ModulusIsZero,
    NoInverse,
    Overflow,
    VectorDimensionMismatch,
    MatrixDimensionMismatch,
    InvalidGroupParameters,
    InvalidSecretKey,
    InvalidNonce,
}

use std::fmt;
impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::ModulusIsZero => write!(f, "modulus P cannot be 0"),
            CryptoError::NoInverse => write!(f, "no modular inverse exists"),
            CryptoError::Overflow => write!(f, "operation did not succeed, overflow resulted."),
            CryptoError::VectorDimensionMismatch => {
                write!(f, "the two vectors are of different lengths.  ")
            }
            CryptoError::MatrixDimensionMismatch => {
                write!(f, "the two matrices are of different lengths.  ")
            }
            CryptoError::InvalidGroupParameters => write!(f, "invalid group parameters"),
            CryptoError::InvalidSecretKey => write!(f, "secret key must be non-zero"),
            CryptoError::InvalidNonce => write!(f, "encryption nonce must be non-zero"),
        }
    }
}

impl std::error::Error for CryptoError {}
