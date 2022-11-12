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
    public List<Integer> inorderTraversal(TreeNode root) {
        List<Integer> return_list = new ArrayList<Integer>();
        helperFunc(root, return_list);
        return return_list;
    }

    private void helperFunc(TreeNode root, List<Integer> return_list) {
        if (root == null) {
            return;
        }
        helperFunc(root.left, return_list);
        return_list.add(root.val);
        helperFunc(root.right, return_list);
    }
}