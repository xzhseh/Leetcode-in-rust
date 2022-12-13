class Solution {
    public String getHint(String secret, String guess) {
        int[] secret_count = new int[10];
        int[] guess_count = new int[10];
        boolean[] flag = new boolean[secret.length()];
        int bulls_count = 0;
        int cows_count = 0;

        for (int i = 0; i < secret.length(); ++i) {
            flag[i] = false;
        }

        for (int i = 0; i < secret.length(); ++i) {
            if (secret.charAt(i) == guess.charAt(i)) {
                bulls_count += 1;
                flag[i] = true;
            }
        }

        for (int i = 0; i < secret.length(); ++i) {
            if (!flag[i]) {
                secret_count[secret.charAt(i) - '0'] += 1;
                guess_count[guess.charAt(i) - '0'] += 1;
            }
        }

        for (int i = 0; i < 10; ++i) {
            cows_count += Math.min(secret_count[i], guess_count[i]);
        }

        return Integer.toString(bulls_count) + "A" + Integer.toString(cows_count) + "B";
    }
}