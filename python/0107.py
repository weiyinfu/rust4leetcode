# Definition for a binary tree node.
# class TreeNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution(object):
    def __init__(self):
        self.depth=0
    def levelOrderBottom(self, root):
        """
        :type root: TreeNode
        :rtype: List[List[int]]
        """
        a=[]
        def go(r,dep):
            if not r:return
            self.depth=max(self.depth,dep)
            a.append((r.val,dep,len(a)))
            go(r.left,dep+1)
            go(r.right,dep+1)
        go(root,0)
        depth=self.depth
        ans=[[]for i in range(depth+1)]
        for i in a:
            ans[i[1]].append(i)
        for i in range(depth+1):
            ans[i]=sorted(ans[i],key=lambda x:x[2])
            ans[i]=map(lambda x:x[0],ans[i])
        ret=[]
        for i in ans:
            if i:
                ret.append(i)
        return ret[::-1]