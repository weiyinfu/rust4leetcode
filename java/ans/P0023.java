package leetcode.p0023;

import java.util.Comparator;
import java.util.PriorityQueue;

class ListNode {
int val;
ListNode next;

ListNode(int x) {
}
}

class Solution {

public ListNode mergeKLists(ListNode[] lists) {
    PriorityQueue<ListNode> queue = new PriorityQueue<>(new Comparator<ListNode>() {
        @Override
        public int compare(ListNode o1, ListNode o2) {
            return o1.val - o2.val;
        }
    });
    for (ListNode i : lists) {
        if (i == null) continue;
        queue.add(i);
    }
    ListNode head = new ListNode(0);
    ListNode now = head;
    while (queue.size() > 0) {
        ListNode node = queue.poll();
        if (node.next != null) {
            queue.add(node.next);
        }
        now.next = node;
        now = now.next;
    }
    return head.next;
}
}