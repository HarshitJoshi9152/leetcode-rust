use std::collections::*;

#[allow(dead_code)]
fn is_valid_sudoku(sudoku: Vec<Vec<char>>) -> bool
{
	let mut row_set : [HashSet<char>; 9] = [
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
	];
	let mut col_set : [HashSet<char>; 9] = [
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
	];
	let mut grid_set : [HashSet<char>; 9] = [
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
		HashSet::with_capacity(9),
	];

	for (ridx, row) in sudoku.iter().enumerate() {
		for (cidx, elm) in row.iter().enumerate() {
			// calulate the grid set
			if *elm == '.' { continue;}
			let grid_idx = (ridx / 3) * 3 + (cidx / 3);

			if row_set[ridx].contains(elm) || col_set[cidx].contains(elm) || grid_set[grid_idx].contains(elm) {
				return false;
			}
			row_set[ridx].insert(*elm);
			col_set[cidx].insert(*elm);
			grid_set[grid_idx].insert(*elm);
		}
	}

	return true;
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_is_valid_sudoku()
	{

		let sudoku = vec![vec!['5','3','.','.','7','.','.','.','.']
								 ,vec!['6','.','.','1','9','5','.','.','.']
								 ,vec!['.','9','8','.','.','.','.','6','.']
								 ,vec!['8','.','.','.','6','.','.','.','3']
								 ,vec!['4','.','.','8','.','3','.','.','1']
								 ,vec!['7','.','.','.','2','.','.','.','6']
								 ,vec!['.','6','.','.','.','.','2','8','.']
								 ,vec!['.','.','.','4','1','9','.','.','5']
								 ,vec!['.','.','.','.','8','.','.','7','9']];
		
		let sudoku2 = vec![vec!['8','3','.','.','7','.','.','.','.'],
												vec!['6','.','.','1','9','5','.','.','.'],
												vec!['.','9','8','.','.','.','.','6','.'],
												vec!['8','.','.','.','6','.','.','.','3'],
												vec!['4','.','.','8','.','3','.','.','1'],
												vec!['7','.','.','.','2','.','.','.','6'],
												vec!['.','6','.','.','.','.','2','8','.'],
												vec!['.','.','.','4','1','9','.','.','5'],
												vec!['.','.','.','.','8','.','.','7','9']];

		let test_cases = [
			(sudoku, true),
			(sudoku2, false)
		];

		for (input, exp_result) in test_cases {
			assert_eq!(
				super::is_valid_sudoku(input), exp_result
			);
		}
	}
}