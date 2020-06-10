class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        if not nums: return 0
        i = 1
        j = 0
        while i < len(nums):
            if nums[i] == nums[j]:
                i += 1
                continue
            else:
                j += 1
                nums[j] = nums[i]
                i += 1
        return j + 1
