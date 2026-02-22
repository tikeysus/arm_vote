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