------------------
title = "leetcode44:wildcard"
publishTime = "2018-03-01 09:02:00"
id = "8489354"
tags = [ "算法", "leetcode",]

--------------

[44. Wildcard Matching](https://leetcode.com/problems/wildcard-matching/description/)

## 问题描述
给定字符串s和模式p，判断字符串s是否完全符合模式p
其中字符串s只包含小写字母，模式串p包含小写字母、`*`、`?`，其中星号表示任意长度的任意字符串，问号表示任意一个字符（不能是空）。

## 解决思路
这么小的问题，不至于使用正则表达式。
即便使用正则表达式可以解决，那也肯定不如特殊问题特殊处理速度快、效率高。
这个问题中‘？’还好解决，关键是星号。一种很直观的思路就是：a=p.split("*")，将字符串p使用星号进行分隔，得到一个字符串数组a，只要s中顺次包含a中的全部字符串，那就必然匹配成功。
这么实现需要注意的点是：当`s="mabcd",p="ab*cd"`时，s确实包含ab和cd，但是ab必须得作为开头存在，否则程序就出错了。最好的解决方法就是添加哨兵单元，在字符串s和p的首尾各加上一个特殊字符，如`$`,`^`等。

```python
class Solution:
    def isMatch(self, s, p):
        """
        :type s: str
        :type p: str
        :rtype: bool
        """
        a = []
        now = ''
        p = '^' + p + "$"
        s = '^' + s + '$'
        for i in p:
            if i == '*':
                if len(now):
                    a.append(now)
                    now = ''
            else:
                now = now + i
        if len(now):
            a.append(now)
        i = 0
        j = 0
        while i < len(s) and j < len(a):
            if self.ok(s, i, a[j]):
                i += len(a[j])
                j += 1
            else:
                i += 1
        return j == len(a) and i == len(s)

    def ok(self, s, i, ss):
        if i + len(ss) > len(s): return False
        for j in range(len(ss)):
            if ss[j] != s[i + j] and not ss[j] == '?':
                return False
        return True


if __name__ == '__main__':
    s = Solution()
    for i in (("a", "*"),
              ("acb", '*a?b*'),
              (
                      "babbbbaabababaabbababaababaabbaabababbaaababbababaaaaaabbabaaaabababbabbababbbaaaababbbabbbbbbbbbbaabbb",
                      "b**bb**a**bba*b**a*bbb**aba***babbb*aa****aabb*bbb***a")):
        print(i[0], i[1], s.isMatch(i[0], i[1]))

```
        