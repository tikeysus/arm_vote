
//this function causes subtraction with overflow, gotta find out why. 
pub fn egcd(a: u64, b: u64) -> (i128, i128, i128){
	if b == 0 {
        return (a as i128, 1, 0);
    }

    let (d, x1, y1) = egcd(b, a % b); 

    let x = y1;
    let y = x1 - y1 * ( (a / b) as i128 );

    (d, x, y)
}

pub fn iterative_egcd(_a: u64, _b: u64) -> (u64, u64, u64){
	todo!(); 
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_egcd_base_case() {
		// When b = 0, should return (a, 1, 0)
		let (d, x, y) = egcd(42, 0);
		assert_eq!(d, 42);
		assert_eq!(x, 1);
		assert_eq!(y, 0);
	}

	#[test]
	fn test_egcd_base_case_zero() {
		let (d, x, y) = egcd(0, 0);
		assert_eq!(d, 0);
		assert_eq!(x, 1);
		assert_eq!(y, 0);
	}

	#[test]
	fn test_egcd_same_number() {
		let (d, x, y) = egcd(5, 5);
		assert_eq!(d, 5);
		// Verify Bézout's identity: a*x + b*y = gcd
		assert_eq!(5 * x + 5 * y, d);
	}

	#[test]
	fn test_egcd_coprime() {
		// For coprime numbers, gcd should be 1
		let (d, x, y) = egcd(7, 3);
		assert_eq!(d, 1);
		// Verify Bézout's identity
		assert_eq!(7 * x + 3 * y, d);
	}

	#[test]
	fn test_egcd_basic() {
		let (d, x, y) = egcd(240, 46);
		assert_eq!(d, 2);
		// Verify Bézout's identity: 240*x + 46*y = 2
		assert_eq!(240 * x + 46 * y, d);
	}

	#[test]
	fn test_egcd_return_gcd() {
		// Test that first element matches standard gcd
		let (d1, _, _) = egcd(48, 18);
		assert_eq!(d1, 6);

		let (d2, _, _) = egcd(54, 24);
		assert_eq!(d2, 6);

		let (d3, _, _) = egcd(101, 103);
		assert_eq!(d3, 1);
	}

	#[test]
	fn test_egcd_with_one() {
		let (d, x, y) = egcd(1, 1);
		assert_eq!(d, 1);
		assert_eq!(1 * x + 1 * y, d);

		let (d2, x2, y2) = egcd(100, 1);
		assert_eq!(d2, 1);
		assert_eq!(100 * x2 + 1 * y2, d2);
	}

	#[test]
	fn test_egcd_larger_numbers() {
		let (d, x, y) = egcd(1071, 462);
		assert_eq!(d, 21);
		// Verify Bézout's identity
		assert_eq!(1071 * x + 462 * y, d);
	}

	#[test]
	fn test_egcd_primes() {
		let (d, x, y) = egcd(17, 13);
		assert_eq!(d, 1);
		assert_eq!(17 * x + 13 * y, d);

		let (d2, x2, y2) = egcd(23, 19);
		assert_eq!(d2, 1);
		assert_eq!(23 * x2 + 19 * y2, d2);
	}

	#[test]
	fn test_egcd_multiples() {
		let (d, x, y) = egcd(15, 10);
		assert_eq!(d, 5);
		assert_eq!(15 * x + 10 * y, d);

		let (d2, x2, y2) = egcd(21, 14);
		assert_eq!(d2, 7);
		assert_eq!(21 * x2 + 14 * y2, d2);
	}
}