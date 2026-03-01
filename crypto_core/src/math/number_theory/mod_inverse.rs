use crate::modint::const_modint::ConstModInt; 
use crate::errors::CryptoError; 
use crate::math::number_theory::egcd::egcd;

pub fn modular_inverse<const P: u64>(a: ConstModInt<P>) -> Result<ConstModInt<P>, CryptoError> {
	let (d, x, _) = egcd(a.value(), P);
    if d != 1 {
        return Err(CryptoError::NoInverse);
    }

	//if x is negative, x % P will still be negative
	let mut inv = x % ( P as i128 ); 
		if inv < 0{
			inv += P as i128;
		}
	let inv = inv as u64; 

    ConstModInt::new(inv) 
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_modular_inverse_prime_modulus() {
		// With prime modulus 7
		let a = ConstModInt::<7>::new(3).unwrap();
		let inv = modular_inverse(a).unwrap();
		// 3 * 5 = 15 ≡ 1 (mod 7)
		assert_eq!((3 * inv.value()) % 7, 1);
	}

	#[test]
	fn test_modular_inverse_one() {
		// Inverse of 1 is always 1
		let a = ConstModInt::<11>::new(1).unwrap();
		let inv = modular_inverse(a).unwrap();
		assert_eq!(inv.clone().value(), 1);
		assert_eq!((1 * inv.clone().value()) % 11, 1);
	}

	#[test]
	fn test_modular_inverse_large_prime() {
		// With prime modulus 101
		let a = ConstModInt::<101>::new(38).unwrap();
		let inv = modular_inverse(a).unwrap();
		// Verify a * inv ≡ 1 (mod 101)
		assert_eq!((38 * inv.value()) % 101, 1);
	}

	#[test]
	fn test_modular_inverse_coprime_composite() {
		// With composite modulus 15, inverse of 2
		let a = ConstModInt::<15>::new(2).unwrap();
		let inv = modular_inverse(a).unwrap();
		// 2 * 8 = 16 ≡ 1 (mod 15)
		assert_eq!((2 * inv.value()) % 15, 1);
	}

	#[test]
	fn test_modular_inverse_various_values() {
		// Test several values with modulus 13 (prime)
		for i in 1..13 {
			let a = ConstModInt::<13>::new(i).unwrap();
			let inv = modular_inverse(a).unwrap();
			assert_eq!((i * inv.value()) % 13, 1, "Failed for i={}", i);
		}
	}

	#[test]
	fn test_modular_inverse_no_inverse_gcd_not_one() {
		// 6 and 9 are not coprime (gcd = 3)
		let a = ConstModInt::<9>::new(6).unwrap();
		let result = modular_inverse(a);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), CryptoError::NoInverse);
	}

	#[test]
	fn test_modular_inverse_no_inverse_divisible() {
		// 10 is divisible by 5, so no inverse mod 10
		let a = ConstModInt::<10>::new(5).unwrap();
		let result = modular_inverse(a);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), CryptoError::NoInverse);
	}

	#[test]
	fn test_modular_inverse_no_inverse_even_mod() {
		// 4 and 8 share factor 4
		let a = ConstModInt::<8>::new(4).unwrap();
		let result = modular_inverse(a);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), CryptoError::NoInverse);
	}

	#[test]
	fn test_modular_inverse_zero() {
		// 0 has no modular inverse
		let a = ConstModInt::<7>::new(0).unwrap();
		let result = modular_inverse(a);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), CryptoError::NoInverse);
	}

	#[test]
	fn test_modular_inverse_fermat_little_theorem() {
		// For prime p, a^(p-1) ≡ 1 (mod p)
		// So a^(p-2) ≡ a^(-1) (mod p)
		// Test with p=17, a=5
		let a = ConstModInt::<17>::new(5).unwrap();
		let inv = modular_inverse(a).unwrap();
		
		// Compute 5^15 mod 17 using our pow function
		let a_pow = ConstModInt::<17>::new(5).unwrap().pow(15).unwrap();
		
		// Both should give same result (mod 17)
		assert_eq!((5 * inv.clone().value()) % 17, 1);
		assert_eq!((5 * a_pow.value) % 17, 1);
	}

	#[test]
	fn test_modular_inverse_pairwise() {
		// Test that inverse is symmetric: inv(inv(a)) = a
		let a = ConstModInt::<11>::new(7).unwrap();
		let inv1 = modular_inverse(a).unwrap();
		let b = ConstModInt::<11>::new(inv1.clone().value).unwrap();
		let inv2 = modular_inverse(b).unwrap();
		assert_eq!(inv2.clone().value() % 11, 7);
	}
}