use std::collections::HashMap;

#[allow(dead_code)]
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();

    for n in nums {
        map.entry(n)
            .and_modify(|(_, f)| { *f += 1 })
            .or_insert((n, 1));
    }

    let mut vals: Vec<_> = map.into_values().collect();

		// TODO: will the performance / complexity here increase if we use a heap ?
		// if we use a heap the sort time / insertion time to heap is o(log k) But we have to perform that action n - k times
		// fi we use quick sort or merge sort the time is o (log n)
		// I wonder which approach is better, I should time both functions on different types of inputs
    vals.sort_unstable_by(| (_, f), (_, f1)  |  f1.partial_cmp(f).unwrap() );

    return vals.into_iter().take(k as usize).map(|(n, _)| n).collect();
}

#[cfg(test)]
mod tests {
    #[test]
	fn test_func() {
		let test_cases = [
			((vec![1,1,1,2,2,3], 2), [1, 2])
		];

		for (input, exp_result) in test_cases {
			let (nums, k) = input;
			assert_eq!(
				super::top_k_frequent(nums, k), exp_result
			);
		}

	}
}