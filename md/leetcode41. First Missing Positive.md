------------------
title = "leetcode41. First Missing Positive"
publishTime = "2018-03-04 15:33:00"
id = "8506854"
tags = [ "算法", "leetcode",]

--------------

给定一个数组，找出数组中不曾出现的最小正整数。

关键在于需要对原数组进行操作。

```python
class Solution:
    def firstMissingPositive(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        if not nums: return 1
        for i in range(len(nums)):
            while 0 < nums[i] < len(nums) and nums[nums[i] - 1] != nums[i]:
                temp = nums[i]
                nums[i] = nums[temp - 1]
                nums[temp - 1] = temp
        print(nums)
        for i in range(len(nums)):
            if nums[i] != i + 1:
                return i + 1
        return len(nums)+1

```
        