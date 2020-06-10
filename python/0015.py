import  bisect
class Solution(object):
    def threeSum(self, nums):
        """
        :type nums: List[int]
        :rtype: List[List[int]]
        """
        nums=sorted(nums)
        ans=[]
        for i in range(len(nums)):
            if nums[i]>0:break
            if i>0 and nums[i-1]==nums[i]:continue
            for j in range(i+1,len(nums)):
                if nums[j]>-nums[i]:break
                if j>i+1 and nums[j]==nums[j-1]:continue
                pos=bisect.bisect_left(nums,0-nums[i]-nums[j],lo=j+1)
                if pos<len(nums) and nums[pos]+nums[i]+nums[j]==0:
                    ans.append([nums[i],nums[j],nums[pos]])
        return ans