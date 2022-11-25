pub struct Solution;

/* time: 100.00% space: 34.78% */
impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i = 0;
        let mut j = matrix[0].len() - 1;
        let row_up = matrix.len() - 1;
        let col_down = 0;

        loop {
            if matrix[i][j] > target {
                if j != col_down {
                    j -= 1;
                } else {
                    return false;
                }
            } else if matrix[i][j] < target {
                if i != row_up {
                    i += 1;
                } else {
                    return false;
                }  
            } else {
                break;
            }
        }

        true
    }
}