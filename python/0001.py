class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        ans=[]
        for i in range(len(nums)):
            if nums[i+1:].__contains__(target-nums[i]):
                ans.append(i)
                two=nums[i+1:].index(target-nums[i])+i+1
                ans.append(two)
                break
        return ans

