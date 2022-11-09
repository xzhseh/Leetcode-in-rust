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
    public boolean hasCycle(ListNode head) {
        if (head == null || head.next == null) {
            return false;
        }
        ListNode ptr1 = head.next; // The fast ptr
        ListNode ptr2 = head; // The slow ptr
        while (ptr1 != ptr2) {
            if (ptr1 == null || ptr1.next == null) {
                return false;
            } else {
                ptr1 = ptr1.next.next;
                ptr2 = ptr2.next;
            }
        }
        return true;
    }
}