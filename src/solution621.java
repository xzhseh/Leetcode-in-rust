class Solution {
    /* time: 23.39% space: 76.67% */ 
    public int leastInterval(char[] tasks, int n) {
        int[] fre_list = new int[30];
        int[] cd_list = new int[30];
        boolean[] record_list = new boolean[30];
        int total_tasks = tasks.length; // Already execute the first task
        int total_min_time = 0; // return value

        for (int i = 0; i < 30; ++i) {
            record_list[i] = false;
        }

        for (char cur_char : tasks) { // First iteration through tasks
            fre_list[cur_char - 'A'] += 1;
            record_list[cur_char - 'A'] = true;
        }

        for (int i = 0; i < 30; ++i) { // Initialization of the cd_list
            cd_list[i] = 0;
        }

        do {
            int index = 0;
            int min = Integer.MAX_VALUE; // Hard-coded here, just make sure it's big enough
            int max_frequency = -1; // Also Hard-coded :)

            for (int i = 0; i < 30; ++i) { // Find the current smallest one with the biggest frequency
                if (!record_list[i]) {
                    continue;
                }

                if (cd_list[i] <= min && fre_list[i] > 0) {
                    if (cd_list[i] == min) {
                        if (fre_list[i] > max_frequency) {
                            max_frequency = fre_list[i];
                            index = i;
                        }
                    } else if (cd_list[i] < min) { // To make thing more precise here
                        min = cd_list[i];
                        max_frequency = fre_list[i];
                        index = i;
                    }
                }
            }

            total_min_time += (min + 1);
            cd_list[index] = n;
            fre_list[index] -= 1;
            total_tasks -= 1;

            for (int i = 0; i < 30; ++i) { // Update some relevant parameters
                if (!record_list[i]) {
                    continue;
                }

                if (i == index) {
                    continue;
                }

                if (cd_list[i] > (min + 1)) {
                    cd_list[i] -= (min + 1);
                } else {
                    cd_list[i] = 0;
                }
            }
        } while (total_tasks != 0);

        return total_min_time;
    }
}