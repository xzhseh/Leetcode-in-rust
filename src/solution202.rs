pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;
        let mut current_set: HashSet<i32> = HashSet::new();
        let mut current_val: i32 = n;
        let mut flag = true;

        loop {
            if current_val == 1 {
                break;
            } else {
                if current_set.contains(&current_val) {
                    flag = false;
                    break;
                } else {
                    current_set.insert(current_val);
                    current_val = Solution::cal_square(current_val);
                }
            }
        }

        flag
    }

    #[allow(dead_code)]
    pub fn cal_square(mut n: i32) -> i32 {
        let mut return_val: i32 = 0;

        while n != 0 {
            let current_val = n % 10;
            return_val += current_val * current_val;
            n /= 10;
        }

        return_val
    }
}