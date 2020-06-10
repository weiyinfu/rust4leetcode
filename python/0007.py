class Solution(object):
    def reverse(self, x):
        """
        :type x: int
        :rtype: int
        """
        if x==0:return 0
        sig=x//abs(x)
        ans=int(str(x)[::-1].strip('-'))*sig
        if ans>2**31-1 or ans<-2**31:
            return 0
        else:
            return ans
