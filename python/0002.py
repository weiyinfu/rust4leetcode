# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        a = ''
        b = ''
        while l1:
            a += str(l1.val)
            l1=l1.next
        a=a[::-1]
        while l2:
            b += str(l2.val)
            l2=l2.next
        b=b[::-1]
        a = int(a)
        b = int(b)
        c = list(str(a + b))[::-1]
        c=[int(i)for i in c]
        return c
