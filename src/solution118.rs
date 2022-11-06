pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut return_vec: Vec<Vec<i32>> = Vec::new();
		for i in 0..num_rows {
			let mut tmp_vec: Vec<i32> = vec![1; (i + 1) as usize]; // Fill in one for the initialization
            if i < 2 { 
                return_vec.push(tmp_vec);
                continue; 
            }
			for j in 1..i {
				tmp_vec[j as usize] = return_vec[(i - 1) as usize][(j - 1) as usize] + 
                                      return_vec[(i - 1) as usize][j as usize];
			}
			return_vec.push(tmp_vec);
		}
		return_vec
    }
}