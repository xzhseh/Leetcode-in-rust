pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut state = 0;
        let mut current_num = 1;
        let mut i = 0;
        let mut j = 0;

        let mut up = 0;
        let mut down = n - 1;
        let mut left = 0;
        let mut right = n - 1;

        let mut return_vec: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

        loop {
            return_vec[i][j] = current_num;

            match state {
                0 => {
                    if j != right as usize {
                        j += 1;
                    } else {
                        i += 1;
                        state = 1;
                        up += 1;
                    }
                },
                1 => {
                    if i != down as usize {
                        i += 1;
                    } else {
                        j -= 1;
                        state = 2;
                        right -= 1;
                    }
                },
                2 => {
                    if j != left as usize {
                        j -= 1;
                    } else {
                        i -= 1;
                        state = 3;
                        down -= 1;
                    }
                },
                3 => {
                    if i != up as usize {
                        i -= 1;
                    } else {
                        j += 1;
                        state = 0;
                        left += 1;
                    }
                },
                _ => (),
            }

            if current_num == n * n {
                break;
            }

            current_num += 1;
        }

        return_vec
    }
}