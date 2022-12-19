/* time: 67.96% space: 68.13 */
class Solution {
    public int[] findBall(int[][] grid) {
        int num_of_balls = grid[0].length;
        int[] return_list = new int[num_of_balls];

        for (int i = 0; i < num_of_balls; ++i) {
            int cur_col = i;

            for (int[] row : grid) {
                int cur_dir = row[cur_col];
                cur_col += cur_dir;

                if (cur_col < 0 || cur_col >= num_of_balls || row[cur_col] != cur_dir) {
                    return_list[i] = -1;
                    break;
                }
            }

            if (return_list[i] != -1) {
                return_list[i] = cur_col;
            }
        }

        return return_list;
    }
}