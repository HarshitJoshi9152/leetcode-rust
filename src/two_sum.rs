
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		
		let mut nums : Vec<(usize, i32)> = nums.into_iter().enumerate().collect();

		nums.sort_unstable();

		let mut left = 0;
		let mut right = nums.len() - 1;
		let mut sum = nums[left].1 + nums[right].1;

		while sum != target
		{
				if target > sum {
						left += 1;
				} else {
						right -= 1;
				}
				sum = nums[left].1 + nums[right].1;
		}

		return vec![nums[left].0 as i32, nums[right].0 as i32];
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_two_sum()
	{
		let test_cases = [
			((vec![2, 7, 11, 13], 9), [0, 1])
		];

		for (input, exp_result) in test_cases {
			let (nums, k) = input;
			assert_eq!(
				super::two_sum(nums, k), exp_result
			);
		}
	}
}