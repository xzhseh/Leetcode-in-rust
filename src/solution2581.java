class Solution {
    // Greedy Algorithm
    public int leastMinutes(int n) {
        int cur_speed = 1;
        int return_num = 0;

        while (n > 0) {
            if (n <= cur_speed) {
                n -= cur_speed;
                return_num += 1;
            } else {
                cur_speed *= 2;
                return_num += 1;
            }
        }

        return return_num;
    }

    // Also available of Dynamic Programming
    private int dp(int n) {
        ArrayList<Integer> dp = new ArrayList<Integer>();
        // Make sure dp[1] = 1
        dp.add(0);
        dp.add(1);

        for (int i = 2; i <= n; ++i) {
            dp.add(dp.get((i + 1) / 2) + 1);
        }

        return dp.get(n);
    }
}