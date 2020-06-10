import math


class Solution:
    def searchRange(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        if not nums:
            return [-1, -1]
        l = 0
        r = len(nums)
        while l + 1 < r:
            m = (l + r) >> 1
            if nums[m] >= target:
                r = m
            elif nums[m] < target:
                l = m
        if nums[l] < target:
            l += 1
        if l >= len(nums) or nums[l] != target:
            return [-1, -1]
        r = len(nums) - 1
        step = 1 << int(math.log2(max(1, r - l)))
        while nums[r] > target and step:
            if r > step and nums[r - step] > target:
                r -= step
            step >>= 1
        if nums[r] > target:
            r -= 1
        return [l, r]


if __name__ == '__main__':
    print(Solution().searchRange([1, 2, 3, 3, 3, 3, 4, 5, 9], 3))
