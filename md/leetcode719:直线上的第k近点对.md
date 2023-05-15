------------------
title = "leetcode719:直线上的第k近点对"
publishTime = "2017-10-29 10:12:00"
id = "7750653"
tags = [ "算法", "leetcode",]

--------------

##问题描述
给定数组a[N],可以确定C(N,2)个点对，也就确定了C(N,2)个距离，求这些距离中第k小的距离（k<C(N,2)）。

##思路
看到第k小、第k大这种问题，首先想到二分法。
把求值问题转化为：小于这个值的元素有多少个。

这道题的区间问题需要仔细考虑清楚，在所有出现小于号的地方考虑是否需要带等号。

##代码
```python
class Solution:
    def smallestDistancePair(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: int
        """
        nums = sorted(nums)
        l = 0
        r = nums[-1] - nums[0]

        def find(m):
            s = 0
            j = 0
            for i in range(len(nums)):
                while j < len(nums) and nums[j] - nums[i] <= m:
                    j += 1
                s += j - i - 1
            return s

        while l< r:
            m = (l + r) // 2
            s = find(m)
            if s >= k:
                r = m
            else:
                l = m+1
        return l
```
        