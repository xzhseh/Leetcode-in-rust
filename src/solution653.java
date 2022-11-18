/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    public boolean findTarget(TreeNode root, int k) {
        HashSet<Integer> hashset = new HashSet<Integer>();
        return traverse(root, k, hashset);
    }

    private boolean traverse(TreeNode root, int t, HashSet<Integer> hashset) {
        if (root == null) {
            return false;
        }

        if (hashset.contains(t - root.val)) {
            return true;
        }
        hashset.add(root.val);
        return traverse(root.left, t, hashset) || traverse(root.right, t, hashset);
    }
}