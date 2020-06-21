class Solution:
    def singleNumber(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        x = [0] * 32
        for i in nums:
            if i < 0:
                i = -i
                x[31] += 1
            for j in range(32):
                if i & (1 << j):
                    x[j] += 1
        for i in range(32):
            x[i] %= 3
        s = 0
        for i in range(31):
            if x[i]:
                s |= (1 << i)
        if x[31]:
            s *= -1
            if s == 0:
                s = -(1 << 31)
        return s


if __name__ == '__main__':
    print(Solution().singleNumber([-1, -2, -2, -2, -3, -3, -3]))
