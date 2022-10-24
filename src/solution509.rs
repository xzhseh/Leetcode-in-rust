pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn fib(n: i32) -> i32 {
        let mut mem_vec: Vec<i32> = vec![-1; (n + 10) as usize];
        mem_vec[0] = 0;
        mem_vec[1] = 1;
        Solution::recursion(n, &mut mem_vec)
    }

    #[allow(dead_code)]
    pub fn recursion(n: i32, mem_vec: &mut Vec<i32>) -> i32 {
        if mem_vec[n as usize] != -1 {
            mem_vec[n as usize]
        } else {
            mem_vec[n as usize] = Solution::recursion(n - 1, mem_vec) + Solution::recursion(n - 2, mem_vec);
            mem_vec[n as usize]
        }
    }
}