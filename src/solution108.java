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
    public TreeNode sortedArrayToBST(int[] nums) {
        return traverse(nums, 0, nums.length - 1);
    }

    private TreeNode traverse(int[] current_vec, int left, int right) {
        if (left > right) { // Check for the special case
            return null;
        } 

        int mid = (left + right) / 2;

        TreeNode new_node = new TreeNode(current_vec[mid]);
        new_node.left = traverse(current_vec, left, mid - 1);
        new_node.right = traverse(current_vec, mid + 1, right);
        
        return new_node;
    }
}