------------------
title = "leetcode621 贪心：任务安排"
publishTime = "2017-09-21 11:20:00"
id = "7570556"
tags = [ "算法", "leetcode",]

--------------

[题目链接](https://leetcode.com/problems/task-scheduler/description/)

给定26种任务，每种任务的数量已知。
相同任务之间必须间隔n个时间段，为了不足n个时间段，可以让及其休息。
问：最少需要多长时间才能处理完这些任务？

这道题用贪心策略解决：每次安排任务时，优先安排任务数比较多的。
实现上，按照批次执行任务，n+1作为一个任务周期。执行完每批任务之后，根据每个任务的数量对当前任务进行排序。
需要注意细节：最后一批任务是不需要加上休息时间的。

```python3
class Solution:
    def leastInterval(self, tasks, n):
        def all0(a):  # 全为0
            for i in a:
                if i != 0:
                    return False
            return True

        if not tasks: return 0
        ma = {}
        for i in tasks:
            if i not in ma:
                ma[i] = 0
            ma[i] += 1
        a = sorted(ma.values(), key=lambda x: -x)
        if n == 0: return sum(a)
        ans = 0
        while 1:
            for i in range(min(len(a), n + 1)):
                a[i] -= 1
            if all0(a):#如果全为0，那就肯定不需要休息了
                ans += len(a)
                return ans
            ans += (n + 1)
            #重新排序
            a = sorted(a, key=lambda x: -x)
            na = []
            for i in a:
                if i:
                    na.append(i)
                else:
                    break
            a = na

```
        