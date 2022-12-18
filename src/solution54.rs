pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut return_vec: Vec<i32> = Vec::new();
        let mut count = 0;
        let mut top: usize = 0;
        let mut bottom: usize = row - 1;
        let mut right: usize = col - 1;
        let mut left: usize = 0;
        let mut row_index: usize = 0;
        let mut col_index: usize = 0;
        let mut status: usize = 0; // 0, 1, 2, 3 represent four different ways

        loop {
            if count == row * col {
                break;
            }

            return_vec.push(matrix[row_index][col_index]);

            if status == 0 {
                if col_index == right {
                    status = 1;
                    row_index += 1;
                    top += 1;
                } else {
                    col_index += 1;
                }
            } else if status == 1 {
                if row_index == bottom {
                    status = 2;
                    col_index -= 1;
                    right -= 1;
                } else {
                    row_index += 1;
                }
            } else if status == 2 {
                if col_index == left {
                    status = 3;
                    row_index -= 1;
                    bottom -= 1;
                } else {
                    col_index -= 1;
                }
            } else if status == 3 {
                if row_index == top {
                    status = 0;
                    col_index += 1;
                    left += 1;
                } else {
                    row_index -= 1;
                }
            }

            count += 1;
        }

        return_vec
    }
}