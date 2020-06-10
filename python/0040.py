import math


class Solution:
    def __init__(self):
        self.ans = []

    def solve(self, candidates, target, beg, now):
        if beg == len(candidates):
            if target == 0:
                self.ans.append(now)
            return
        for i in range(min(candidates[beg][1] + 1, 1 + int(math.ceil(target / candidates[beg][0])))):
            self.solve(candidates, target - candidates[beg][0] * i, beg + 1, now + [candidates[beg][0]] * i)

    def combinationSum2(self, candidates, target):
        """
        :type candidates: List[int]
        :type target: int
        :rtype: List[List[int]]
        """
        self.ans.clear()
        candidates = sorted(candidates)
        ca = []
        i = 0
        while i < len(candidates):
            now = candidates[i]
            cnt = 0
            while i < len(candidates) and candidates[i] == now:
                cnt += 1
                i += 1
            ca.append((now, cnt))
        self.solve(ca, target, 0, [])
        return self.ans


if __name__ == '__main__':
    print(Solution().combinationSum2([1, 1, 6], 8))
