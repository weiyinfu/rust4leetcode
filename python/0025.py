
class Solution:
    def reverse(self, node, k):
        nex = node
        for i in range(k):
            if nex is None:
                return node, None
            nex = nex.next
        now = node
        prev = nex
        for i in range(k):
            temp = now.next
            now.next = prev
            prev = now
            now = temp
        return prev, node

    def reverseKGroup(self, head, k):
        """
        :type head: ListNode
        :type k: int
        :rtype: ListNode
        """
        if k == 1:
            return head
        if not head:
            return head
        ans = self.reverse(head, k)
        now = ans[1]
        while now and now.next:
            no = self.reverse(now.next, k)
            now.next = no[0]
            now = no[1]
        return ans[0]

