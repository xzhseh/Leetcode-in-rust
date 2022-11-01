pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut joseph_circle: Vec<i32> = (1..=n).collect();
        let mut ptr: i32 = 0;
        let mut _return_val: i32 = -1;
        loop {
            if joseph_circle.len() == 1 {
                _return_val = joseph_circle[0];
                break;
            }

            ptr = (ptr + k - 1) % (joseph_circle.len() as i32);
            joseph_circle.remove(ptr as usize);
        }

        _return_val
    }
}