pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: i32 = (nums1.len() - 1) as i32;
        let mut m: i32 = m - 1;
        let mut n: i32 = n - 1;

        loop {
            if n < 0 {
                break;
            }

            while m >= 0 && (nums1[m as usize] > nums2[n as usize]) {
                let _tmp = nums1[i as usize];
                nums1[i as usize] = nums1[m as usize];
                nums1[m as usize] = _tmp;
                i -= 1;
                m -= 1;
            }
            
            std::mem::swap(&mut nums1[i as usize], &mut nums2[n as usize]);
            i -= 1;
            n -= 1;
        } 
    }
}