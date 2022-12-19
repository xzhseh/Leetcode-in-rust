/* time: 100.00% space: 87.69% for recursion */
/* time: 100.00% space: 87.92% for iteration */
class Solution {
    int[] record_list = new int[40];
    // The recursion way of the solution
    public int tribonacci(int n) {
        if (record_list[n] != 0 && n != 0) {
            return record_list[n];
        } else if (n == 0) {
            return record_list[0];
        } else if (n == 1) {
            record_list[1] = 1;
            return record_list[1];
        } else if (n == 2) {
            record_list[2] = 1;
            return record_list[2];
        } else {
            record_list[n] = tribonacci(n - 3) + tribonacci(n - 2) + tribonacci(n - 1);
            return record_list[n];
        }
    }

    // The iteration way of the solution via dp
    private int iteration(int n) {
        if (n == 0) {
            return 0;
        } else if (n <= 2) {
            return 1;
        }

        int first = 0;
        int second = 1;
        int third = 1;
        int result = 2;

        for (int i = 4; i <= n; ++i) {
            first = second;
            second = third;
            third = result;
            result = first + second + third;
        }

        return result;
    }
}