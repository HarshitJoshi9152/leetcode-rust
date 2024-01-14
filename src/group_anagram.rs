use std::collections::HashMap;

#[allow(dead_code)]
fn anagram_hash(a: &String) -> [i32; 26]
{
	// All letters are lowercase so 65..=90
	let mut flags = [0; 26];
	a.bytes().for_each(|b| flags[(b-97) as usize] += 1);
	flags
}

#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>
{
	let mut groups: HashMap<[i32; 26], Vec<String>> = HashMap::new();

	for s in strs {
		let hash = anagram_hash(&s);
		groups.entry(hash)
			.and_modify(|v| { (*v).push(s.clone()) } )
			.or_insert(vec![s.clone()]);
	}
	return groups.into_values().collect();
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_group_anagrams()
	{
		let mut test_cases = [
			(vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()], vec![vec!["bat".to_string()],vec!["nat".to_string(),"tan".to_string()],vec!["ate".to_string(),"eat".to_string(),"tea".to_string()]])
		];

		for (input, exp_result) in test_cases.iter_mut() {
			let strs = input;
			// We cannot move out of a `&mut`
			let mut actual_result = super::group_anagrams(strs.clone());

			// Sort Both results to get same ordering
			sort_nested_arr(&mut actual_result);
			sort_nested_arr(exp_result);

			assert_eq!(
				actual_result, *exp_result
			);
		}
	}

	fn sort_nested_arr(arr: &mut Vec<Vec<String>>) {
		for sub_array in arr.iter_mut() {
			sub_array.sort_unstable();
		}
		arr.sort_unstable();
	}
}