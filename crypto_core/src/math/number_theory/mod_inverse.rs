use crate::modint::const_modint::ConstModInt; 
use crate::errors::CryptoError; 
use crate::math::number_theory::egcd::egcd;
use crate::math::number_theory::gcd::gcd; 

pub fn modular_inverse<const P: u64>(a: ConstModInt<P>) -> Result<u64, CryptoError> {
	let value = a.value; 
    if gcd(value, P) != 1{
		return Err(CryptoError::NoInverse); 
	}
     
	let (_d, x, _y) = egcd(a.value.into(), P.into()); 
	Ok(x) 
}