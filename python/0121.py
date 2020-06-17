class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        mi=0xfffffff
        ans=0
        for i in prices:
            ans=max(ans,i-mi)
            mi=min(i,mi)
        return ans