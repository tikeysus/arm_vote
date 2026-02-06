pub struct ConstModInt<const P: u64>{
    value: u64,
}

impl<const P: u64> ConstModInt<P>{
	pub fn new(x: u64) -> Self{
		self { value: x % P }
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

	pub fn print_modulus() {
		println!("Modulus is {}", P);
	}

}