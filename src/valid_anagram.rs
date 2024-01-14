
#[allow(dead_code)]
fn valid_anagrams(s: String, t: String) -> bool {
	s.len() == t.len() && {
		let mut c = [0; 256];
		s.bytes().for_each(|b| c[b as usize] += 1);
		t.bytes().for_each(|b| c[b as usize] -= 1);
		c.iter().all(|&c| c == 0)
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn test_valid_anagrams()
	{
		let test_cases = [
			(("rat", "car"), false),
			(("anagram", "nagaram"), true)
		];

		for (input, exp_result) in test_cases {
			let (s1, s2) = input;
			assert_eq!(
				super::valid_anagrams(s1.to_string(), s2.to_string()), exp_result
			);
		}
	}
}