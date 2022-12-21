/* This is for LCP-07 */ 
class Solution {
    int ways, n, k;
    List<List<Integer>> edges;

    public int numWays(int n, int[][] relation, int k) {
        ways = 0;
        this.n = n;
        this.k = k;
        edges = new ArrayList<List<Integer>>();

        for (int i = 0; i < n; ++i) {
            edges.add(new ArrayList<Integer>());
        }

        for (int[] edge : relation) {
            int src = edge[0];
            int dst = edge[1];
            edges.get(src).add(dst);
        }

        dfs(0, 0);

        return ways;
    }

    private void dfs(int index, int steps) {
        if (steps == k) {
            if (index == n - 1) {
                ways += 1;
            }
            return;
        }
        List<Integer> edge = edges.get(index);
        for (int next_stop : edge) {
            dfs(next_stop, steps + 1);
        }
    }
}