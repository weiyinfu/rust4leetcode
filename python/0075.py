class Solution:
    def sortColors(self, nums):
        """
        :type nums: List[int]
        :rtype: void Do not return anything, modify nums in-place instead.
        """
        r, g, b = 0, 0, 0
        for i in nums:
            if i == 0:
                r += 1
            elif i == 1:
                g += 1
            elif i == 2:
                b += 1
        j = 0
        for i in range(r):
            nums[j] = 0
            j += 1
        for i in range(g):
            nums[j] = 1
            j += 1
        for i in range(b):
            nums[j] = 2
            j += 1
