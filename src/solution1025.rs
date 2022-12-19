pub struct Solution;

/* time: 100.00% space: 100.00% */
impl Solution {
    #[allow(dead_code)]
    pub fn divisor_game(n: i32) -> bool {
        // When n is even, then who's the first should win, vice versa.
        n % 2 == 0
    }
}