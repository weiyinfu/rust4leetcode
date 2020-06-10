# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def mergeTwoLists(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        beg=1
        ans=None
        now=None
        while 1:
            if l2 and not l1 or (l1 and l2 and l1.val>l2.val):
                if beg:
                    beg=0
                    ans=ListNode(l2.val)
                    now=ans
                else:
                    now.next=ListNode(l2.val)
                    now=now.next
                l2=l2.next
            elif l1 and not l2 or (l1 and l2 and l2.val>=l1.val):
                if beg:
                    beg=0
                    ans=ListNode(l1.val)
                    now=ans
                else:
                    now.next=ListNode(l1.val)
                    now=now.next
                l1=l1.next
            elif not l1 and not l2:
                return ans