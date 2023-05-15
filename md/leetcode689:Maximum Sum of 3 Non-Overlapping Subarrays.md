------------------
title = "leetcode689:Maximum Sum of 3 Non-Overlapping Subarrays"
publishTime = "2017-10-07 14:27:00"
id = "7636061"
tags = [ "算法", "leetcode",]

--------------

给定数组a[N]（每个元素都是正整数）和一个整数k（k小于等于N/3），要求从数组a中找出不相交的三个数组，每个数组长度都为k，使得三个数组之和最大。输出(i,j,k)表示三个子数组的开始下标，如果有多个答案，返回最小的那个三元组。

分析：
这个问题是前缀和的“花式玩法”，也可以看做是动态规划。
定义数组s[N],s[i]表示sum(a[i-k+1]~a[i])
定义数组ss[N],ss[i]表示sum(a[i-k+1]~a[i])+max(s[0~(i-k)])，也就是i前面的两个片段最大和，且第二个片段以i结尾。
定义数组sss[N]，sss[i]表示i前面的三个片段最大和，且第二个片段以i结尾。
这个问题时空复杂度都为O(N)。

这个问题还有一种简洁的解法，原因在于3的特殊性。
什么是“三”，三就是左边一片，右边一片，中间一片。
定义left数组，left[i]表示i左面最大的片段
定义right数组，right[i]表示i右面最大的片段
定义ans数组，ans[i]为中间一片、左边一片、右边一片之和，也就是ans[i]=s[i]+left[i-k]+right[i+1]

任何事物，如果要想找到它的简便方法，就必须应用上这个事物的特殊性。

```python
class Solution:
    def maxSumOfThreeSubarrays(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: List[int]
        """
        # print(nums)
        #前缀和
        s = [0] * len(nums)
        s[0] = nums[0]
        for i in range(1, len(s)):
            s[i] = s[i - 1] + nums[i]

        # print('s', s) 
        a = [0] * len(nums)
        a[k - 1] = s[k - 1]
        for i in range(k, len(s)):
            a[i] = s[i] - s[i - k]
        # print('a', a)
        #最大前缀和
        ss = [0] * len(nums)
        ma = 0
        for i in range(k - 1, len(s)):
            if a[i] > a[ma]:
                ma = i
            ss[i] = (a[ma], ma)
        # print('ss',ss)
        sss = [0] * len(nums)
        for i in range(k * 2 - 1, len(s)):
            sss[i] = a[i] + ss[i - k][0]
        # print('sss',sss)
        #二级最大前缀和
        b = [0] * len(nums)
        ma = 0
        for i in range(k * 2 - 1, len(s)):
            if sss[i] > sss[ma]:
                ma = i
            b[i] = (sss[ma], ma)
        # print('b',b)
        #三级前缀和
        c = [0] * len(nums)
        for i in range(k * 3 - 1, len(s)):
            c[i] = a[i] + b[i - k][0]
        # print('c',c)
        ans = 0
        for i in range(k * 3 - 1, len(c)):
            if c[i] > c[ans]:
                ans = i
        ret = [0, 0, ans]
        ret[1] = b[ret[2] - k][1]
        ret[0] = ss[ret[1] - k][1]
        ret = list(map(lambda i: i - k+1, ret))
        return ret


if __name__ == '__main__':
    ans = Solution().maxSumOfThreeSubarrays([1,2,1,2,6,7,5,1], 2)
    print(ans)

```
        