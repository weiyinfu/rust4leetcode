class Solution:
    def trap(self, height):
        """
        :type height: List[int]
        :rtype: int
        """
        height = [0] + height + [0]
        left = [0] * len(height)
        right = [0] * len(height)
        ma = 0
        for i in range(len(height)):
            left[i] = ma
            ma = max(ma, height[i])
        ma = 0
        for i in range(len(height) - 1, -1, -1):
            right[i] = ma
            ma = max(ma, height[i])
        s = 0
        for i in range(len(height)):
            s += max(0,min(left[i], right[i]) - height[i])
        return s