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
    public TreeNode insertIntoBST(TreeNode root, int val) {
        if (root == null) {
            return new TreeNode(val);
        }
        helper_func(root, val);
        return root;
    }
    
    private void helper_func(TreeNode root, int val) {
        if (root.left == null && val < root.val) {
            root.left = new TreeNode(val);
            return;
        } else if (root.right == null && val > root.val) {
            root.right = new TreeNode(val);
            return;
        } 
        
        if (root.val > val) {
            helper_func(root.left, val);
        } else {
            helper_func(root.right, val);
        }
    }
}