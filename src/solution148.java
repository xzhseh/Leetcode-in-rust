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
    public ListNode sortList(ListNode head) {
        return sort(head, null);
    }

    private ListNode sort(ListNode head, ListNode tail) {
        if (head == null) {
            return head;
        } else if (head.next == tail) {
            head.next = null;
            return head;
        }

        ListNode slow = head;
        ListNode fast = head;
        while (fast != tail) {
            slow = slow.next;
            fast = fast.next;
            if (fast != tail) {
                fast = fast.next;
            }
        }

        ListNode list_one = sort(head, slow);
        ListNode list_two = sort(slow, tail);
        ListNode merge_list = mergeTwoLists(list_one, list_two);

        return merge_list;
    }

    private ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        // The recursion way of doing this, the price of recursion is more overhead
        if (list1 == null) { // The base case of recursion
            return list2;
        } else if (list2 == null) {
            return list1;
        } else if (list1.val < list2.val) {
            ListNode tmp_node = list1;
            tmp_node.next = mergeTwoLists(list1.next, list2);
            return tmp_node;
        } else {
            ListNode tmp_node = list2;
            tmp_node.next = mergeTwoLists(list1, list2.next);
            return tmp_node;
        }
    }
}