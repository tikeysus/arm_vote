pub fn gcd(mut a: u64, mut b: u64) -> u64{
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_gcd_basic() {
		assert_eq!(gcd(48, 18), 6);
		assert_eq!(gcd(18, 48), 6); // commutative
		assert_eq!(gcd(54, 24), 6);
		assert_eq!(gcd(101, 103), 1);
	}

	#[test]
	fn test_gcd_zero() {
		assert_eq!(gcd(0, 0), 0);
		assert_eq!(gcd(0, 5), 5);
		assert_eq!(gcd(5, 0), 5);
		assert_eq!(gcd(0, 100), 100);
	}

	#[test]
	fn test_gcd_same_number() {
		assert_eq!(gcd(42, 42), 42);
		assert_eq!(gcd(1, 1), 1);
		assert_eq!(gcd(100, 100), 100);
	}

	#[test]
	fn test_gcd_one() {
		assert_eq!(gcd(1, 1), 1);
		assert_eq!(gcd(1, 100), 1);
		assert_eq!(gcd(100, 1), 1);
	}

	#[test]
	fn test_gcd_powers_of_two() {
		assert_eq!(gcd(8, 12), 4);
		assert_eq!(gcd(16, 24), 8);
		assert_eq!(gcd(32, 48), 16);
		assert_eq!(gcd(64, 128), 64);
	}

	#[test]
	fn test_gcd_large_numbers() {
		assert_eq!(gcd(1071, 462), 21);
		assert_eq!(gcd(123456, 789012), 12);
		assert_eq!(gcd(9999999, 3333333), 3333333);
	}

	#[test]
	fn test_gcd_primes() {
		assert_eq!(gcd(17, 19), 1);
		assert_eq!(gcd(23, 29), 1);
		assert_eq!(gcd(97, 101), 1);
	}

	#[test]
	fn test_is_coprime_true() {
		assert!(is_coprime(1, 1));
		assert!(is_coprime(3, 5));
		assert!(is_coprime(7, 11));
		assert!(is_coprime(15, 28));
		assert!(is_coprime(17, 19));
		assert!(is_coprime(100, 101));
	}

	#[test]
	fn test_is_coprime_false() {
		assert!(!is_coprime(2, 4));
		assert!(!is_coprime(6, 9));
		assert!(!is_coprime(10, 15));
		assert!(!is_coprime(12, 18));
		assert!(!is_coprime(100, 50));
	}

	#[test]
	fn test_is_coprime_with_zero() {
		assert!(!is_coprime(0, 0));
		assert!(!is_coprime(0, 5));
		assert!(!is_coprime(5, 0));
	}

	#[test]
	fn test_lcm_basic() {
		assert_eq!(lcm(4, 6), 12);
		assert_eq!(lcm(6, 4), 12); // commutative
		assert_eq!(lcm(3, 5), 15);
		assert_eq!(lcm(12, 18), 36);
	}

	#[test]
	fn test_lcm_same_number() {
		assert_eq!(lcm(5, 5), 5);
		assert_eq!(lcm(42, 42), 42);
		assert_eq!(lcm(1, 1), 1);
	}

	#[test]
	fn test_lcm_with_one() {
		assert_eq!(lcm(1, 5), 5);
		assert_eq!(lcm(5, 1), 5);
		assert_eq!(lcm(1, 100), 100);
	}

	#[test]
	fn test_lcm_coprime() {
		assert_eq!(lcm(7, 11), 77);
		assert_eq!(lcm(13, 17), 221);
		assert_eq!(lcm(3, 5), 15);
	}

	#[test]
	fn test_lcm_powers_of_two() {
		assert_eq!(lcm(4, 6), 12);
		assert_eq!(lcm(8, 12), 24);
		assert_eq!(lcm(16, 24), 48);
	}

	#[test]
	fn test_lcm_large_numbers() {
		assert_eq!(lcm(21, 6), 42);
		assert_eq!(lcm(48, 18), 144);
		assert_eq!(lcm(100, 150), 300);
	}
} 