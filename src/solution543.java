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
    private int return_val = 0;

    public int diameterOfBinaryTree(TreeNode root) {
        depth(root);
        return return_val - 1;
    }

    private int depth(TreeNode cur_node) {
        if (cur_node == null) {
            return 0;
        }

        int left_depth = depth(cur_node.left);
        int right_depth = depth(cur_node.right);
        return_val = Math.max(return_val, left_depth + right_depth + 1);

        return Math.max(left_depth, right_depth) + 1;
    }
}