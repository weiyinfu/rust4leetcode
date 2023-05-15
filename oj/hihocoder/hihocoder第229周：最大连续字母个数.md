------------------
title = "hihocoder第229周：最大连续字母个数"
publishTime = "2018-11-18 03:54:00"
id = "9977430"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho229/problems)

给定一个仅包含小写字母的字符串s（长度小于1e5），你可以交换任意两个字符的位置，现在允许交换k次，要求交换之后，s中最长的连续相同字符个数尽量多，求这个最长连续区间的长度。

样例
```plain
输入
1  ：表示k
bababbaa：表示s
输出
4
```
只需要把s[0]处的b移动到s[3]，能够达成长度为4的连续区间。

# 思路
小写字母只有26种，这是一个重要信息。最后的答案会是哪个小写字母“达成”的呢？ 只需要枚举26种小写字母。

最后的答案会是在哪个位置达成的呢？只需要枚举|s|个起始位置。

因为枚举连续区间起始位置的时候，连续区间是谁达成的就已经确定了（显然是由连续区间的第一个字符达成的），所以只需要枚举|s|个起始位置。

当起始位置为beg时，只需要求出连续区间的end来，从beg到end总共需要移动的次数是end-beg+1-从beg到end已经存在了的字符的个数。需要移动的次数需要小于等于k，关键在于寻找满足约束的end，这个过程可以二分实现，从beg到end已经存在的字符个数可以用前缀和数组O（1）实现。


下面代码是错误的
bbaaaabbbbbbbb，这种样例无法通过。
双向扫描一遍才能通过。

```java
import java.util.Scanner;

public class Main {
int[][] dp;
char[] a;
int[] count;
int k;

int needMove(int beg, int end) {
    int ch = a[beg] - 'a';
    int nowCount = dp[end][ch] - dp[beg][ch] + 1;
    int regionLength = end - beg + 1;
    int move = regionLength - nowCount;
    return move;
}

int maxContinue(int ind) {
    //以ind开头移动k次最多能够达成的最大连续个数
    int left = ind, right = Math.min(ind + count[a[ind] - 'a'] - 1, a.length - 1);
    while (left + 1 < right) {
        int mid = (left + right) >> 1;
        int move = needMove(ind, mid);
        if (move > k) {
            right = mid - 1;
        } else if (move < k) {
            left = mid + 1;
        } else {
            left = mid;
        }
    }
    int rightMove = needMove(ind, right);
    int end = left;
    if (rightMove <= k) end = right;
    return end - ind + 1;
}

int solve() {
    int ans = 0;
    for (int i = 0; i < a.length; i++) {
        if (i > 0 && a[i] == a[i - 1]) {
            continue;
        }
        ans = Math.max(ans, maxContinue(i));
    }
    return ans;
}

Main() {
    Scanner cin = new Scanner(System.in);
    k = cin.nextInt();
    a = cin.next().trim().toCharArray();
    dp = new int[a.length][27];
    for (int i = 0; i < dp[0].length; i++) {
        dp[0][i] = 0;
    }
    dp[0][a[0] - 'a'] = 1;
    for (int i = 1; i < a.length; i++) {
        System.arraycopy(dp[i - 1], 0, dp[i], 0, dp[0].length);
        dp[i][a[i] - 'a'] += 1;
    }
    count = dp[a.length - 1];
    System.out.println(solve());
}

public static void main(String[] args) {
    new Main();
}
}
```

# 优化：双指针单向移动枚举beg和end
如果一个区间[beg,end]是合法的（移动k次能够达成连续），那么这个区间的子区间也是合法的。这个原理保证了end向后单调移动而不会回溯。

当移动end时，我们只需要判断end能否向后移动（区间[beg+1,end+1]是否合法）。这个问题跟第一种思路中的判断移动次数原理是一样的，此法需要枚举[beg,end]区间上每种字符是否合法，复杂度为`|s|*26`。
        