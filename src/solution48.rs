pub struct Solution;

/* time: 100.00% space: 70.00% */
impl Solution {
    #[allow(dead_code)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut tmp_matrix = matrix.clone();
        let col = matrix[0].len() - 1;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                tmp_matrix[j][col - i] = matrix[i][j];
            }
        }
        *matrix = tmp_matrix.clone();
    }
}

