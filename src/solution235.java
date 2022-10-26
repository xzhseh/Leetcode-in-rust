/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */

class Solution {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        List<TreeNode> p_array = new ArrayList<TreeNode>();
        List<TreeNode> q_array = new ArrayList<TreeNode>();
        traverse(root, p, p_array);
        traverse(root, q, q_array);
        int index = 0;
        for (int i = 0; i < Math.min(p_array.size(), q_array.size()); ++i) {
            if (p_array.get(i) == q_array.get(i)) {
                    index = i;
            } else {
                break;
            }
        }
        return p_array.get(index);
    }

    private void traverse(TreeNode root, TreeNode target, List<TreeNode> path) {
        if (root == target) {
            path.add(root);
            return;
        } 
        path.add(root);
        if (root.val > target.val) {
            traverse(root.left, target, path);
        } else {
            traverse(root.right, target, path);
        }
    }
}