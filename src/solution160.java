/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode(int x) {
 *         val = x;
 *         next = null;
 *     }
 * }
 */

public class Solution {
    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        HashSet<ListNode> current_set = new HashSet<ListNode>();

        while (headA != null) {
            current_set.add(headA);
            headA = headA.next;
        }

        while (headB != null) {
            if (current_set.contains(headB)) {
                return headB;
            } else {
                current_set.add(headB);
                headB = headB.next;
            }
        }

        return null;
    }   
}