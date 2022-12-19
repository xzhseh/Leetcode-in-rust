class Solution {
    int max = -1;
    int[] record_list = new int[110];
    public int getMaximumGenerated(int n) {
        if (n == 0) {
            return 0;
        } else if (n == 1) {
            return 1;
        } else if (n == 2) {
            return 1;
        } else {
            record_list[0] = 0;
            record_list[1] = 1;
            for (int i = 2; i <= n; ++i) {
                if (i % 2 == 0) {
                    record_list[i] = record_list[i / 2];
                } else {
                    record_list[i] = record_list[(i - 1) / 2] + record_list[(i - 1) / 2 + 1];
                    if (record_list[i] > max) {
                        max = record_list[i];
                    }
                }
            }

            return max;
        }
    }
}