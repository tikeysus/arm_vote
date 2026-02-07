use crate::errors::CryptoError; 

pub struct ConstModInt<const P: u64>{
    value: u64,
}

impl<const P: u64> ConstModInt<P>{
	pub fn new(x: u64) -> Result<Self, CryptoError>{
		if P == 0 { 
			return Err(CryptoError::ModulusIsZero); 
		}
		ok(Self { value: x % P }) 
	}

	pub fn value(self) -> u64{
		self.value
	}

	pub fn add(self, other: Self) -> Self{
		let result = (self.value + other.value) % P; 
		Self::new(result)
	}

	pub fn sub(self, other: Self) -> Self{
		let result = (self.value + P - other.value) % P; 
		Self::new(result)
	}

	pub fn mul(self, other: Self) -> Self{
		let result = (self.value * other.value) % P; 
		Self::new(result) 
	}

	pub fn pow(self, mut exponent: u64) -> Self{
		if exponent == 0 { return Self::new(1) }
		if exponent == 1 { return self }

		let mut result = 1; 
		let mut base = self.value % P; 
		while (exponent > 0){
			if (exponent & 1 == 1){
				result = (result * base) % P; 
			}
			base = (base * base) % P; 
			exponent >>= 1;
		}
		Self::new(result) 
	}

	pub fn print_modulus() {
		println!("Modulus is {}", P);
	}
}