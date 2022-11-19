pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let mut return_num = 0;
        let mut grid = grid.clone();

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    return_num += 1;
                    Solution::dfs(&mut grid, j as i32, i as i32);
                }
            }
        }

        return_num
    }

    #[allow(dead_code)]
    pub fn dfs(current_grid: &mut Vec<Vec<char>>, col: i32, row: i32) {
        let grid_row = current_grid.len();
        let grid_col = current_grid[0].len();

        if col >= grid_col as i32 || row >= grid_row as i32 || col < 0 || row < 0 || current_grid[row as usize][col as usize] == '0' {
            return;
        }

        current_grid[row as usize][col as usize] = '0';

        Solution::dfs(current_grid, col + 1, row);
        Solution::dfs(current_grid, col, row + 1);
        Solution::dfs(current_grid, col - 1, row);
        Solution::dfs(current_grid, col, row - 1);
    }
}