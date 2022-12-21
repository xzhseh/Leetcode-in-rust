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
    public ListNode removeNthFromEnd(ListNode head, int n) {
        if (head.next == null) {
            return null;
        }

        int length = 0;
        int iterate_index = 0; // From 0..=length - n
        ListNode _tmp = head;

        while (_tmp != null) { // Calculate the length of the list
            _tmp = _tmp.next;
            length += 1;
        }

        if (length == n) {
            return head.next;
        }

        ListNode tmp = head;

        while (iterate_index != length - n - 1) {
            tmp = tmp.next;
            iterate_index += 1;
        }

        tmp.next = tmp.next.next;

        return head;
    }
}