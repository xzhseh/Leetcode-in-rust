pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if trust.len() == 0 {
            if n == 1 {
                return 1;
            } else if n == 2 {
                return -1;
            }
        }
        let mut in_degree: Vec<i32> = vec![0; (n + 1) as usize];
        let mut out_degree: Vec<i32> = vec![0; (n + 1) as usize];
        let mut index: i32 = -1;
        let mut return_val = -1;

        for current_vec in trust {
            in_degree[current_vec[1] as usize] += 1;
            out_degree[current_vec[0] as usize] += 1;
            if in_degree[current_vec[1] as usize] == n - 1 && out_degree[current_vec[1] as usize] == 0 {
                index = current_vec[1];
            }
        } 

        if index != -1 && out_degree[index as usize] == 0 {
            return_val = index;
        }

        return_val
    }
}