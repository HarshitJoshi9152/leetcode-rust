
#[allow(dead_code)]
fn product_of_array(nums: Vec<i32>) -> Vec<i32> {
  
  let mut left_products = nums.clone();
  let mut right_products = nums.clone();
  let mut left = 1;
  let mut right = nums.len() - 2;
  loop {
    // I think we can return early if we find a 0 here

    left_products[left] = left_products[left-1] * nums[left];
    right_products[right] = right_products[right+1] * nums[right];
    
    if left == nums.len() - 1 { break }

    left += 1;
    right -= 1;
  }

  // let mut products = Vec::with_capacity(nums.len()); // why doesnt this work lmao
  let mut products = nums.clone();

  products[0] = right_products[1]; // 2nd last Product (taking products from right)
  products[nums.len() - 1] = left_products[nums.len() - 2]; // 2nd Last Product

  let mut i = 1;

  loop {
    if i == nums.len() - 1 {
      break;
    }

    products[i] = left_products[i-1] * right_products[i + 1];
    i += 1;
  }

  products
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_product_of_array()
	{
		let test_cases = [
			((vec![1,2,3,4]), [24, 12, 8, 6])
		];

		for (input, exp_result) in test_cases {
			let nums = input;
			assert_eq!(
				super::product_of_array(nums), exp_result
			);
		}
	}
}