pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
		let total_elem = mat.len() * mat[0].len();
		let mut return_matrix: Vec<Vec<i32>> = Vec::new();
		if total_elem as i32 != r * c { return mat; }
		let mut r_ptr = 0;
		let mut c_ptr = 0;
		for _i in 0..r {
			let mut tmp_matrix: Vec<i32> = Vec::new();
			for _j in 0..c {
				tmp_matrix.push(mat[r_ptr][c_ptr]);
				if c_ptr == mat[0].len() - 1 {
					r_ptr += 1;
					c_ptr = 0;
				} else {
                    c_ptr += 1;
                }
			}
			return_matrix.push(tmp_matrix);
		}
		return_matrix			
    }
}