class Solution(object):
    def removeElement(self, nums, val):
        """
        :type nums: List[int]
        :type val: int
        :rtype: int
        """
        if not nums:return 0
        i=0
        for j in range(len(nums)):
            if nums[j]==val:
                continue
            else:
                nums[i]=nums[j]
                i+=1
        return i