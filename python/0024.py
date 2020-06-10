# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def swapPairs(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if not head: return
        if not head.next: return head
        ans = ListNode(0)
        ans.next = head
        now = ans
        while now.next:
            first = now.next
            if not first:
                break
            second = first.next
            if not second:
                break
            third = second.next
            now.next = second
            now.next.next = first
            now.next.next.next = third
            now = now.next.next
        return ans.next
