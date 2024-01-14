
#[allow(dead_code)]
fn is_palindrome(s: String) -> bool {
	let s= s.chars().filter(|c|
		{
			if !c.is_ascii_alphanumeric() || c.is_whitespace() {
				return false;
			}
			true
		}
	).map(|c| c.to_ascii_lowercase() );
	return s.clone().eq(s.rev());

	// for (i, j) in (s.clone()).zip(s.clone().rev()) {
	// 		if i != j {
	// 				return false;
	// 		}
	// }
	// return true;
}


#[cfg(test)]
mod tests {
	#[test]
	fn test_is_palindrome()
	{
		let test_cases = [
			(
				"A man, a plan, a canal: Panama".to_string(),
				true
			),
			(
				"race a car".to_string(),
				false
			)
		];

		for (input, exp_result) in test_cases {
			assert_eq!(
				super::is_palindrome(input), exp_result
			);
		}
	}
}