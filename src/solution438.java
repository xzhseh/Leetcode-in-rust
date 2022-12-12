class Solution {
    public List<Integer> findAnagrams(String s, String p) {
        if (s.length() < p.length()) {
            return new ArrayList<Integer>();
        }

        ArrayList<Integer> return_list = new ArrayList<Integer>();
        int[] p_set = new int[26];
        int[] s_set = new int[26];
        int index = 0;

        for (int i = 0; i < p.length(); ++i) {
            p_set[p.charAt(i) - 'a'] += 1;
            s_set[s.charAt(i) - 'a'] += 1;
        }

        if (Arrays.equals(p_set, s_set)) {
            return_list.add(0);
        }

        for (int i = 0; i < s.length() - p.length(); ++i) {
            s_set[s.charAt(i) - 'a'] -= 1;
            s_set[s.charAt(i + p.length()) - 'a'] += 1;

            if (Arrays.equals(p_set, s_set)) {
                return_list.add(i + 1);
            }
        }

        return return_list;
    }
}