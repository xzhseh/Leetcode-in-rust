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
    public ListNode swapPairs(ListNode head) {
        if (head == null || head.next == null) { // Handle some special cases
            return head;
        } else if (head.next.next == null) {
            ListNode return_node = head.next;
            head.next.next = head;
            head.next = null;
            return return_node;
        }

        int count = 0;
        ListNode count_node = head;
        while (count_node != null) {
            count += 1;
            count_node = count_node.next;
        }

        ListNode ptr_one = head;
        ListNode ptr_two = head;
        ListNode return_node = head.next;

        if (count % 2 == 0) {
            while (ptr_one.next.next != null) { // Find the second last not changed node
                ptr_one = ptr_one.next;
            }
        } else {
            while (ptr_one.next.next.next != null) { // Find the second last not changed node
                ptr_one = ptr_one.next;
            }
        }

        while (true) {
            // Exchange the node
            ListNode tmp_node = ptr_one.next.next;
            ListNode preserve_node = ptr_one.next;
            ptr_one.next.next = ptr_one;
            ptr_one.next = tmp_node;

            if (ptr_one != head) {
                ListNode Tmp_node = head;

                while (Tmp_node.next != ptr_one) {
                    Tmp_node = Tmp_node.next;
                }

                Tmp_node.next = preserve_node;
            }
            
            // Judge the end condition of the loop
            if (ptr_one == head) {
                break;
            }

            // Update the ptr_one via the move of ptr_two
            while (ptr_two.next.next != preserve_node) {
                ptr_two = ptr_two.next;
            }

            ptr_one = ptr_two;
            ptr_two = head;
        }

        return return_node;
    }

    public ListNode swapPairs_recursion_version(ListNode head) {
        if (head == null || head.next == null) {
            return head;
        }
        ListNode newHead = head.next;
        head.next = swapPairs(newHead.next);
        newHead.next = head;
        return newHead;
    }
}