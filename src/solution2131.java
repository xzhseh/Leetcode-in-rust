class Solution {
    public int longestPalindrome(String[] words) {
        HashMap<String, Integer> the_set = new HashMap<String, Integer>();
        int max_length = 0;
        int count = 0;
        int max_fre = Integer.MIN_VALUE;
        int[] tmp_list = new int[26];
        String the_word = max_fre(words);

        for (String word : words) {
            if (word.equals(the_word)) {
                count += 1;
                max_length += 2;
                continue;
            }

            if (word.charAt(0) == word.charAt(1)) {
                if (the_set.containsKey(word)) {
                    the_set.remove(word);
                    count += 2;
                    max_length += 4;
                } else {
                    the_set.put(word, 1);
                }
            } else if (the_set.containsKey(word)) {
                the_set.put(word, the_set.get(word) + 1);
            } else if (the_set.containsKey(reverse(word))) {
                if (the_set.get(reverse(word)) == 1) {
                    the_set.remove(reverse(word));
                } else {
                    the_set.put(reverse(word), the_set.get(reverse(word)) - 1);
                }
                count += 2;
                max_length += 4;
            } else {
                the_set.put(word, 1);
            }
        }

        if (count % 2 != 0) {
            return max_length;
        }

        for (String word : the_set.keySet()) {
            if (word.charAt(0) == word.charAt(1)) {
                max_length += 2;
                break;
            }
        }

        return max_length;
    }

    private String reverse(String s) {
        return new StringBuilder(s).reverse().toString();
    }

    private String max_fre(String[] words) {
        // HashMap to store the frequency of each word
        HashMap<String, Integer> wordFrequency = new HashMap<>();
        // Initialize the maximum frequency and the word with the highest frequency
        int maxFrequency = -1;
        String highestFrequencyWord = "";

        // Iterate through the input array and count the frequency of each word
        for (String word : words) {
            if (word.charAt(0) != word.charAt(1)) {
                continue;
            }
            // If the word is already in the map, increment its frequency by 1
            if (wordFrequency.containsKey(word)) {
                wordFrequency.put(word, wordFrequency.get(word) + 1);
            } else { // If the word is not in the map, add it and set its frequency to 1
                wordFrequency.put(word, 1);
            }
            // Update the maximum frequency and the highest frequency word if necessary
            if (wordFrequency.get(word) > maxFrequency) {
                maxFrequency = wordFrequency.get(word);
                highestFrequencyWord = word;
            }
        }

        // Return the highest frequency word
        return highestFrequencyWord;
    }
}