class Solution(object):
    def __init__(self):
        self.ans = []

    def go(self, ind, s, li):
        if ind == len(li):
            self.ans.append(s)
            return
        for i in li[ind]:
            self.go(ind + 1, s + i, li)

    def letterCombinations(self, digits):
        """
        :type digits: str
        :rtype: List[str]
        """
        if not digits: return []
        ma = {
            2: 'abc',
            3: 'def',
            4: 'ghi',
            5: 'jkl',
            6: 'mno',
            7: 'pqrs',
            8: 'tuv',
            9: 'wxyz'
        }
        li = list(map(lambda x: ma[int(x)], digits))
        self.go(0, '', li)
        return self.ans