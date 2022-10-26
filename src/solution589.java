/*
// Definition for a Node.
class Node {
    public int val;
    public List<Node> children;

    public Node() {}

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, List<Node> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
    public List<Integer> preorder(Node root) {
        List<Integer> return_list = new ArrayList<Integer>();
        traverse(root, return_list);
        return return_list;
    }

    private void traverse(Node root, List<Integer> current_list) {
        if (root == null) {
            return;
        } else {
            current_list.add(root.val);
            for (Node current_branch: root.children) {
                traverse(current_branch, current_list);
            }
        }
    }
}