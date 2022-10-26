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
    public List<List<Integer>> levelOrder(TreeNode root) {
        if (root == null) { return new ArrayList<List<Integer>>(); }
        Deque<TreeNode> deque_for_tree = new LinkedList<TreeNode>();
        List<List<Integer>> return_list = new ArrayList<List<Integer>>();
        deque_for_tree.add(root);
        while (deque_for_tree.size() != 0) {
            int current_size = deque_for_tree.size();
            List<Integer> tmp_list = new ArrayList<Integer>();
            for (int i = 0; i < current_size; ++i) {
                TreeNode tmp_node = deque_for_tree.poll();
                if (tmp_node.left != null) { deque_for_tree.offer(tmp_node.left); }
                if (tmp_node.right != null) { deque_for_tree.offer(tmp_node.right); }
                tmp_list.add(tmp_node.val);
            }
            return_list.add(tmp_list);
        }
        return return_list;
    } 
}