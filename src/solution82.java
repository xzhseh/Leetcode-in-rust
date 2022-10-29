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
    public ListNode deleteDuplicates(ListNode head) {
        if (head == null || head.next == null) {
            return head;
        }

        if (containDuplicates(head)) {
            int current_val = head.val;
            while (true) {
                if (head == null) {
                    return null;
                }

                if (current_val != head.val) {
                    if (!containDuplicates(head)) {
                        break;
                    } else {
                        current_val = head.val;
                        continue;
                    }
                }

                head = head.next;
            }
        }

        ListNode ptr_one = head;
        ListNode ptr_two = head;

        while (ptr_one.next != null) {
            if (!containDuplicates(ptr_one.next)) { // No duplicates for the current element
                ptr_one = ptr_one.next;
                continue;
            }

            ptr_two = ptr_one.next;
            
            while (ptr_two.val == ptr_one.next.val) {
                if (ptr_two.next == null) {
                    ptr_one.next = null;
                    return head;
                }
                ptr_two = ptr_two.next;
            }

            ptr_one.next = ptr_two;
        }

        return head;
    }   

    private boolean containDuplicates(ListNode node) {
        if (node == null || node.next == null) {
            return false;
        }

        return (node.val == node.next.val);
    }
}