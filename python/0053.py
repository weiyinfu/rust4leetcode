class Solution(object):
    def maxSubArray(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        s=[0]*len(nums)
        s[0]=nums[0]
        for i in range(1,len(nums)):
            if s[i-1]>0:
                s[i]=s[i-1]+nums[i]
            else:
                s[i]=nums[i]
        return max(s)