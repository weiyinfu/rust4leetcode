class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        0 有票
        1 没票
        """
        ans=0
        for i in range(1,len(prices)):
            ans+=max(0,prices[i]-prices[i-1])
        return ans