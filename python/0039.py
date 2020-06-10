class Solution:
    def __init__(self):
        self.ans = []

    def go(self, candidates, ind, target, now):
        if ind == len(candidates):
            if target == 0:
                self.ans.append(now[:])
            return
        for i in range(target // candidates[ind] + 1):
            self.go(candidates, ind + 1, target - i * candidates[ind], now + [candidates[ind]] * i)

    def combinationSum(self, candidates, target):
        """
        :type candidates: List[int]
        :type target: int
        :rtype: List[List[int]]
        """
        self.go(candidates, 0, target, [])
        return self.ans


if __name__ == '__main__':
    a = []
    print(a + [1] * 0)
