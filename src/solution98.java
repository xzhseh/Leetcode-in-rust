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
    public boolean isValidBST(TreeNode root) {
        return helper_func(root, -2e31, 2e31 - 1);
    }

    private boolean helper_func(TreeNode root, double lower, double higher) {
        if (root == null) {
            return true;
        } else if (root.val <= lower || root.val >= higher) {a
            return false;
        } 
        return helper_func(root.left, lower, root.val) && helper_func(root.right, root.val, higher);
    }
}