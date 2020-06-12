# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution(object):
    def inorderTraversal(self, root):
        """
        :type root: TreeNode
        :rtype: List[int]
        """
        ans=[]
        if not root:return ans
        a=[root]
        while len(a):
            now=a.pop()
            if type(now)==TreeNode:
                if now.right:
                    a.append(now.right)
                a.append(now.val)
                if now.left:
                    a.append(now.left)

            else:
                ans.append(now)
        return ans