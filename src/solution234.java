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
    public boolean isPalindrome(ListNode head) {
        if (head == null || head.next == null) {
            return true;
        }

        int length = 0;
        ListNode _tmp = head;
        while(_tmp != null) {
            _tmp = _tmp.next;
            length += 1;
        }

        ArrayList<Integer> stack = new ArrayList<Integer>();

        int target = 0;
        if (length % 2 == 0) {
            target = length / 2 - 1;
        } else {
            target = (length - 1) / 2 - 1;
        }

        int cnt = 0;
        ListNode tmp = head;
        while (cnt != target + 1) {
            stack.add(tmp.val);
            tmp = tmp.next;
            cnt += 1;
        }

        if (length % 2 != 0) {
            tmp = tmp.next;
        }

        while (tmp != null) {
            if (stack.get(target) != tmp.val) {
                return false;
            } else {
                stack.remove(target);
                target -= 1;
                tmp = tmp.next;
                continue;
            }
        }

        return true;
    }
}