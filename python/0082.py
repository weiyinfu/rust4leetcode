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
        if head.next==None:return head
        if head.next.next==None:
            if head.next.val==head.val:
                return None
            else:
                return head
        else:
            x=ListNode(None)
            x.next=head
            ans=x
            val=None
            while x.next.next:
                if x.next.val==x.next.next.val:
                    x.next=x.next.next
                    val=x.next
                else:
                    if val:
                        val=None
                        x.next=x.next.next
                    else:
                        x=x.next
            if val:
                x.next=None
            return ans.next