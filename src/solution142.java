/**
 * Definition for singly-linked list.
 * class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */

public class Solution {
    public ListNode detectCycle(ListNode head) {
        HashSet<ListNode> current_set = new HashSet<ListNode>();
        
        while (head != null) {
            if (current_set.contains(head)) {
                return head;
            } else {
                current_set.add(head);
                head = head.next;
            }
        }

        return null;
    }
}