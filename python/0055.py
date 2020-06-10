class Solution:
    def canJump(self, nums):
        """
        :type nums: List[int]
        :rtype: bool
        """
        if not nums: return False
        ma = 0
        i = 0
        while i < len(nums) and i < ma + 1:
            ma = max(ma, i + nums[i])
            i += 1
        return ma >= len(nums)-1


if __name__ == '__main__':
    print(Solution().canJump([2, 3, 1, 0, 4]))
