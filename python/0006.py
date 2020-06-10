class Solution(object):
    def convert(self, s, numRows):
        """
        :type s: str
        :type numRows: int
        :rtype: str
        """
        if numRows == 1: return s
        ans = ['' for _ in range(numRows)]
        di = 1
        x = 0
        for i in range(len(s)):
            ans[x] += s[i]
            x += di
            if x == numRows:
                di = -1
                x -= 2
            if x == -1:
                x += 2
                di = 1
        return ''.join(ans)
