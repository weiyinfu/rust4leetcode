class Solution(object):
    def __init__(self):
        self.ans = []

    def generateParenthesis(self, n):
        """
        :type n: int
        :rtype: List[str]
        """

        def go(s, l, used):
            if len(s) >= n * 2:
                self.ans.append(s)
                return
            if l:
                go(s + ")", l - 1, used)
            if used < n:
                go(s + "(", l + 1, used + 1)

        go('', 0, 0)
        return self.ans
