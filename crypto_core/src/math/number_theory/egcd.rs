pub fn egcd(a: u64, b: u64) -> (u64, u64, u64){
	if b == 0{
		return (a, 1, 0); 
	}

	let (d, x1, y1) = egcd(b, a % b); 
	let x = y1; 
	let y = x1 - y1 * (a/b); 

	(d, x, y)
}