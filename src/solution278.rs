// The API isBadVersion is defined for you.
// isBadVersion(version: i32) -> bool;
// to call it use self.isBadVersion(version)

pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid; // The first is in [left, mid]
            } else {
                left = mid + 1; // The first is in [mid + 1, right]
            }
        }

        left
    }
}