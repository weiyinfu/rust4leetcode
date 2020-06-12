# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def deleteDuplicates(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if not head:return
        ans=head
        now=head
        while now.next:
            if now.val==now.next.val:
                now.next=now.next.next
            else:
                now=now.next
        return ans