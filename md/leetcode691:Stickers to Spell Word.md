------------------
title = "leetcode691:Stickers to Spell Word"
publishTime = "2017-10-08 15:05:00"
id = "7639064"
tags = [ "算法", "leetcode",]

--------------

[题目链接](https://leetcode.com/problems/stickers-to-spell-word/description/)
给定一个字符串target和一个字符串数组stickers[]，要求从stickers中选取尽量少的字符串，以这些字符串所包含的字母为原材料，拼出target。
其中：
对于stickers中的每个字符串，可以使用多次。
target和stichers中的字符串字符集全部为小写字母集。

##分析

初看这道题，这是一个整数规划问题。
target为aabbb，stickers为[abb,aab]。
设需要x1个abb，x2个aab，则：
min z=x1+x2

`1*x1+2*x2>=2`(提供的a的个数应该大于target中包含的a的个数)
`2*x1+x2>=3`(提供的b的个数应该大于target中包含的b的个数)
x1和x2为非负整数

原题目等价于整数规划问题，这是一个NP问题。
但是题目给的数据范围有提示：target的长度小于15。

##方法一：暴力搜索
```python
import collections


class Solution(object):
    def minStickers(self, stickers, target):
        # 统计target中各个字符的个数
        t_count = collections.Counter(target)
        # 统计stickers中每个字符串中各个字符的个数，去掉target中不包含的字符
        A = [collections.Counter(sticker) & t_count
             for sticker in stickers]
        # 如果sticker1包含sticker2，那么sticker2是无论如何不会选择的
        # 这个循环必须倒着写，因为要删除元素
        for i in range(len(A) - 1, -1, -1):
            if any(A[i] == A[i] & A[j] for j in range(len(A)) if i != j):
                A.pop(i)

        self.best = len(target) + 1

        def search(ans=0):
            if ans >= self.best: return
            if not A:
                if all(t_count[letter] <= 0 for letter in t_count):
                    self.best = ans
                return
            # A一直进行pop操作，直到A为空的时候，得到答案
            sticker = A.pop()
            # 在当前的target中至多需要多少个sticker，当前的target用t_count来表示
            used = max((t_count[letter] - 1) // sticker[letter] + 1
                       for letter in sticker)
            used = max(used, 0)

            for c in sticker:
                t_count[c] -= used * sticker[c]

            search(ans + used)
            # 实际使用该sticker的个数可以是0~used之间的任意一个数字，都要尝试一遍
            for i in range(used - 1, -1, -1):
                for letter in sticker:
                    t_count[letter] += sticker[letter]
                search(ans + i)

            A.append(sticker)

        search()
        return self.best if self.best <= len(target) else -1


```
##方法二：动态规划

使用Python语言编写动态规划代码时，很容易超时。必须加上一个优化，对stickers进行“瘦身”，如果sticker A完爆sticker B，那么无论如何都不能选择B，这样就可以把B从stickers中删掉。使用C++是不需要这个优化就能通过的。

```python
import collections


class Solution:
    def minStickers(self, stickers, target):
        """
        :type stickers: List[str]
        :type target: str
        :rtype: int
        """
        # 首先，进行一些优化，对stickers进行“瘦身”操作，没有这一步，会超时的
        t_count = collections.Counter(target)
        A = [collections.Counter(sticker) & t_count
             for sticker in stickers]

        for i in range(len(A) - 1, -1, -1):
            if any(A[i] == A[i] & A[j] for j in range(len(A)) if i != j):
                A.pop(i)

        stickers = ["".join(s_count.elements()) for s_count in A]
        # stickers瘦身完毕，开始执行动规
        N = 1 << len(target)
        big = 0xffffff
        dp = [big] * N
        dp[0] = 0
        for i in range(N):
            if dp[i] == big:
                continue
            # 当前状态为i，让当前状态加上每一个sticker，为将来的状态进行赋值
            for j in stickers:
                now = i
                # 怎么让当前状态now+当前sticker j呢？
                # 要让sticker j中的每一个字符都发挥作用
                for k in j:
                    for r in range(len(target)):
                        if now & (1 << r):  # 如果now中已经有了第r个字符，那就不需要了
                            continue
                        # now中没有，并且target中有，sticker j中也有，那就可以应用字符k了
                        if k == target[r]:
                            now |= (1 << r)
                            break
                dp[now] = min(dp[i] + 1, dp[now])
        return -1 if dp[-1] > len(target) else dp[-1]

```

这是一种很常见的、动态规划解决NP问题的模式！每个状态用一个数字表示，然后从0开始循环，把每个状态的答案保存起来。用当前状态+操作产生未来状态，并给未来状态进行赋值。
```python
dp=[1<<N]
dp[0]=0
for cur in range(1<<N):#当前状态
    for op in operations:#所有操作
        nextState,value=apply(cur,op)#对当前状态应用操作
        dp[nextState]=min(value,dp[nextState])#更新将来的状态

```

从下面往上面赋值要比从上面往下面赋值简单清晰的多！
更关键的是，有时候只能自下而上的更新状态，因为上面根本不知道自己前面有哪些状态。
这个数组dp虽然是一维数组，实际上它是一个有向无环图。如果状态A可以推出状态B，那么就有一条从A指向B的有向边。将此一维数组一展开，就是一个有向无环图。在这个有向无环图中，寻找某个节点的后继结点可能很容易，而求解某个结点的前向结点就很复杂（关键是要找到全部的前向结点）。这种问题就像二极管一样，从一个方向到另一个方向可以很容易通过，从另一方向回来就难了。就像中国象棋中的马一样，跳过去容易，跳回来难。但是，不排除有时候寻找前向结点的可能性。也许，有些问题就是对状态减去某个操作比较简单。
```python
dp=[1<<N]
dp[0]=0 
for cur in range(1,1<<N):
    for op in operations:
        lastState=sub(cur,op)
        dp[cur]=min(dp[lastState]+1,dp[cur])
```

##方法三：一种超时算法
统计字符个数之后进行动规。
动规的关键在于如何定义每一个状态，定义完状态之后，状态之间通过状态转移来联系起来，就会产生一个有向无环图。
方法二中，没有对输入的各个字符串进行处理，导致循环中需要循环O(字符串长度)次。如果预先统计出每个字符串中各个字符的个数就可以循环O(字符种类数)次。
```python
import collections
import functools
from queue import Queue


class Solution:
    def minStickers(self, stickers, target):
        """
        :type stickers: List[str]
        :type target: str
        :rtype: int
        """

        def solve(A, b):
            ks = list(b.keys())
            vs = [b[i] for i in ks]
            START = tuple([0] * len(b))  # 当前状态
            ma = dict()
            ma[START] = 0
            states = Queue()  # 状态队列
            states.put(START)
            while states.qsize():
                nowTuple = states.get()
                nowStep = ma[nowTuple]
                # print(nowTuple, nowStep)
                for sticker in A:  # 当前状态加上每一个sticker产生未来的状态
                    state = list(nowTuple)
                    for j, c in enumerate(ks):
                        if c in sticker:
                            state[j] = min(state[j] + sticker[c], vs[j])
                    t = tuple(state)
                    if t not in ma or nowStep + 1 < ma[t]:
                        if t not in ma:
                            states.put(t)
                        ma[t] = nowStep + 1

            return ma[tuple(vs)]

        A = [collections.Counter(i) for i in stickers]
        b = collections.Counter(target)
        # 对stickers进行瘦身
        for i in range(len(A) - 1, -1, -1):
            if any(A[i] == A[i] & A[j] for j in range(len(A)) if i != j):
                A.pop(i)
        # stickers包含的字母集合
        s = functools.reduce(lambda s, i: s | i, A, collections.Counter())
        return -1 if not all(i in s for i in b)else solve(A, b)
```

这种方法虽然超时了，但是我认为当字符串长度足够长的时候，这种算法效率更高些。
        