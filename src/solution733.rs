pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image.clone();
        let the_color = image[sr as usize][sc as usize];
        if the_color == color {
            return image;
        }
        Solution::recursion(&mut image, sr, sc, color, -1, the_color);

        image
    }

    pub fn recursion(target_vec: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, from_dir: i32, from_color: i32) {
        if sr >= target_vec.len() as i32 || sr < 0 || sc < 0 || sc >= target_vec[0].len() as i32 {
            return;
        } else {
            if target_vec[sr as usize][sc as usize] == from_color {
                target_vec[sr as usize][sc as usize] = color;
            } else {
                return;
            }
            if from_dir != 0 {
                Solution::recursion(target_vec, sr - 1, sc, color, 2, from_color);
            } 
            if from_dir != 1 {
                Solution::recursion(target_vec, sr, sc + 1, color, 3, from_color);
            }
            if from_dir != 2 {
                Solution::recursion(target_vec, sr + 1, sc, color, 0, from_color);
            }
            if from_dir != 3 {
                Solution::recursion(target_vec, sr, sc - 1, color, 1, from_color);
            }
        }
    }
}