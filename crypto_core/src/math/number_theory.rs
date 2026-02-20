use crate::errors::CryptoError; 
use crate::modint::const_modint::ConstModInt; 

/*
Maybe I will need to implement an algo that is able to calculate something like 
gcd(a,b,c) using gcd(a, gcd(b,c)). But I don't know if larger inputs are useful as of now. 

*/
pub fn gcd(a: u64, b: u64) -> u64{
	if a == 0 || b == 0{
		return a | b; 
	}
	
	let shift = (a|b).trailing_zeros(); 
	a >>= a.trailing_zeros(); 

	loop{ //this is such a crazy keyword in Rust, but I guess you could just do while true 
		b >>= b.trailing_zeros(); 

		if a > b{
			std::mem::swap(&mut a, &mut b); 
		}
		b -= a; 

		if b == 0{ break }
	}

	a << shift 
}

pub fn is_coprime(a: u64, b: u64) -> bool{
	if gcd(a,b) == 1 { return true }
	else { return false }
}

pub fn lcm(a: u64, b: u64) -> u64{
	(a * b) / gcd(a,b)
} //why not 

pub fn extended_euclid(a: u64, b: u64){
	unimplemented!()
} 

pub fn modular_inverse<const P: u64>(a: ConstModInt<P>) -> Result<u64, CryptoError> {
	let value = a.value; 
    if gcd(value, P) != 1{
		return Err(CryptoError::NoInverse); 
	}
    unimplemented!()
}