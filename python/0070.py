class Solution(object):
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """
        if n<=3:return n
        a=1
        b=2
        c=None
        for i in range(3,n+1):
            c=a+b
            a=b
            b=c
        return c