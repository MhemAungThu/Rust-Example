fn main() {
	fn factorial(num: usize) -> usize {
		return match num {
			0 => 1,
			_ => num * factorial(num -1),
		};
	}
	
	println!("{}", factorial(20));
}

