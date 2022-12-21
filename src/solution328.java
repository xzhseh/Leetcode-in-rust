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
    public ListNode oddEvenList(ListNode head) {
        if (head == null || head.next == null) {
            return head;
        }

        ListNode odd_head = head;
        ListNode even_head = head.next;
        ListNode odd_iter = odd_head;
        ListNode even_iter = even_head;
        ListNode iter = head.next.next;
        int count = 3; // Random number, just assure it's odd.

        while (iter != null) {
            if (count % 2 == 0) { // The even case
                even_iter.next = iter;
                even_iter = even_iter.next;
            } else { // The odd case
                odd_iter.next = iter;
                odd_iter = odd_iter.next;
            }

            count += 1;
            iter = iter.next;
        }

        odd_iter.next = even_head;
        even_iter.next = null;
        return head;
    }
}