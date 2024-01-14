
use std::collections::*;

#[allow(dead_code)]
fn contains_duplicate(nums: Vec<i32>) -> bool
{
			// Pre allocate the hashset with nums.len() size
		let mut set = HashSet::<i32>::with_capacity(nums.len());

		for i in nums {
				if !set.insert(i) {
						return true;
				}
		}
		return false;
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_contains_duplicate()
	{
		let test_cases = [
			(vec![1,2,3,1], true),
			(vec![1,2,3,4], false),
			(vec![1,1,1,3,3,4,3,2,4,2], true),
		];

		for (input, exp_result) in test_cases {
			assert_eq!(
				super::contains_duplicate(input), exp_result
			);
		}
	}
}