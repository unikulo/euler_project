pub fn solve_problem(number_of_divisors: i32) -> i32 {
	let mut a = 1; // iteration
	let mut b = 1;
	loop {
		let mut c = b; // copy
		let mut divisors = Vec::new();
		let mut dividor_count = 0;
		let mut dividor_check = 2;
		while c != 1 {
			while c % dividor_check == 0 {
				c = c / dividor_check;
				dividor_count += 1;
			}
			divisors.push(dividor_count);
			dividor_count = 0;
			dividor_check += 1;
		}
		// calculate number of divisors
		let mut divisions = 1;
		for i in divisors.iter() {
			divisions *= i + 1;
		}
		if divisions >= number_of_divisors {
			return b;
		}
		a += 1;
		b += a;
	}
}
#[test]
fn test() {
	assert_eq!(solve_problem(5), 28);
}
