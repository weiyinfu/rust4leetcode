# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution(object):
    def removeNthFromEnd(self, head, n):
        """
        :type head: ListNode
        :type n: int
        :rtype: ListNode
        """
        cnt = 0
        now = head
        while now:
            now = now.next
            cnt += 1
        cnt -= n
        if cnt == 0:
            return head.next
        now = head
        cnt -= 1
        while cnt:
            now = now.next
            cnt -= 1
        now.next = now.next.next
        return head
