class Solution:
    def longestValidParentheses(self, s):
        """
        :type s: str
        :rtype: int
        """
        left = 0
        beg = 0
        ans = 0
        for i in range(len(s)):
            if s[i] == '(':
                left += 1
            elif s[i] == ')':
                left -= 1
                if left == 0:
                    ans = max(ans, i - beg + 1)
                elif left < 0:
                    beg = i + 1
                    left = 0
        right = 0
        end = len(s)
        for i in range(len(s) - 1, -1, -1):
            if s[i] == ')':
                right += 1
            elif s[i] == '(':
                right -= 1
                if right == 0:
                    ans = max(ans, end - i)
                elif right < 0:
                    end = i
                    right = 0
        return ans


if __name__ == '__main__':
    s = Solution()
    print(s.longestValidParentheses("(())()(()(("))
