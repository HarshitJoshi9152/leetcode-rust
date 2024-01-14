
#[allow(dead_code)]
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
	let len = numbers.len();
	let mut i = 0;
	let mut j = len - 1;

	while i < j
	{
		let diff = target - (numbers[i] + numbers[j]);
		if diff == 0 {
			break;
		}

		if diff > 0 {
			// means the sum is small, we need to increase it, increment left pointer
			i += 1;
		}
		else {
			j -= 1;
		}
	}

	return vec![i as i32 + 1, j as i32 + 1];
}


#[cfg(test)]
mod tests {
	#[test]
	fn test_two_sum()
	{
		let test_cases = [
			(
				(vec![2,7,11,15], 9),
				vec![1,2]
			)
		];

		for (input, exp_result) in test_cases {
			let (nums, k) = input;
			assert_eq!(
				super::two_sum(nums, k ), exp_result
			);
		}
	}
}