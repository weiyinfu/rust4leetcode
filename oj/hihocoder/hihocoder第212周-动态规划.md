------------------
title = "hihocoder第212周-动态规划"
publishTime = "2018-07-25 14:43:00"
id = "9368797"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho212/problems)
```java
import java.util.Scanner;

public class Main {
long mod = (long) (1e9 + 7);
int MAXN = 107;
int a[] = new int[MAXN];
int MA = (int) (1e5 + 7);
int MA2 = MA << 1;
int dp[] = new int[MA2];
int nexDp[] = new int[MA2];
int N, S;


Main() {
    Scanner cin = new Scanner(System.in);
    N = cin.nextInt();
    S = cin.nextInt();
    for (int i = 0; i < N; i++) {
        a[i] = cin.nextInt();
    }
    dp[a[0] + MA] = dp[-a[0] + MA] = 1;
    for (int i = 1; i < N; i++) {
        for (int j = 0; j < MA2; j++) {
            nexDp[j] = 0;
        }
        for (int j = 0; j < MA2; j++) {
            if (dp[j] > 0) {
                nexDp[j + a[i]] += dp[j];
                nexDp[j - a[i]] += dp[j];
                nexDp[j + a[i]] %= mod;
                nexDp[j - a[i]] %= mod;
            }
        }
        int[] temp = dp;
        dp = nexDp;
        nexDp = temp;
    }
    System.out.println(dp[S + MA]);
}

public static void main(String[] args) {
    new Main();
}
}

```
        