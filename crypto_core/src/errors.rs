#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoError {
    ModulusIsZero,
    NoInverse,
}

use std::fmt;
impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::ModulusIsZero => write!(f, "modulus P cannot be 0"),
            CryptoError::NoInverse => write!(f, "no modular inverse exists"),
        }
    }
}

impl std::error::Error for CryptoError {}
