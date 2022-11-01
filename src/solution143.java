/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public void reorderList(ListNode head) {
        if (head == null) {
            return;
        }

        List<ListNode> current_list = new ArrayList<ListNode>();
        ListNode tmp_node = head;

        while (tmp_node != null) {
            current_list.add(tmp_node);
            tmp_node = tmp_node.next;
        }
        
        int i = 0;
        int j = current_list.size() - 1;

        while (i < j) {
            current_list.get(i).next = current_list.get(j);
            i += 1;

            if (i == j) {
                break;
            } else {
                current_list.get(j).next = current_list.get(i);
                j -= 1;
            }
        }

        current_list.get(j).next = null;
    }
}