pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals.clone();
        let total_intervals = intervals.len();
        let mut max_not_overlap_intervals = 1;
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut current_bound = intervals[0][1];

        for i in 1..intervals.len() {
            if intervals[i][0] >= current_bound {
                max_not_overlap_intervals += 1;
                current_bound = intervals[i][1];
            } else {
                continue;
            }
        }

        (total_intervals - max_not_overlap_intervals) as i32
    }
}