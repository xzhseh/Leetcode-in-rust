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
    private List<String> return_list = new ArrayList<String>();

    public List<String> binaryTreePaths(TreeNode root) {
        traverse(root, "");
        return return_list;
    }

    private void traverse(TreeNode root, String cur_string) {
        if (root.left == null && root.right == null) {
            return_list.add(cur_string + Integer.toString(root.val));
            return;
        } else if (root.left == null) {
            traverse(root.right, cur_string + Integer.toString(root.val) + "->");
            return;
        } else if (root.right == null) {
            traverse(root.left, cur_string + Integer.toString(root.val) + "->");
            return;
        } else {
            traverse(root.left, cur_string + Integer.toString(root.val) + "->");
            traverse(root.right, cur_string + Integer.toString(root.val) + "->");
            return;
        }
    }
}