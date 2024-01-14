use std::collections::*;

#[allow(dead_code)]
fn leetcode_challenge_name() -> bool
{
	return true;
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_leetcode_challenge_name()
	{
		let test_cases = [
			(
				(),
				[1, 2]
			)
		];

		for (input, exp_result) in test_cases {
			let (nums, k) = input;
			assert_eq!(
				super::leetcode_challenge_name(nums, k), exp_result
			);
		}
	}
}