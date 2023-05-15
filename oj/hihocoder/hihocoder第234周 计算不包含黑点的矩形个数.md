------------------
title = "hihocoder234周 计算不包含黑点的矩形个数"
publishTime = "2018-12-23 07:38:00"
id = "10164492"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho234)

# 问题描述
一个棋盘有n条横线，m条竖线，上面有k个黑点，问有多少个不包含黑点的矩形。

数据范围：
n和m最大为1000，k最大为10

# 方法一：动态规划
复杂度`n*m*k`。

```java
import java.awt.Point;
import java.util.Comparator;
import java.util.LinkedList;
import java.util.List;
import java.util.Scanner;

public class Main {
int n, m;
long dp[][];
List<Point> black;

long solve() {
    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= m; j++) {
            int now = 0;//now表示以i，j这个点为右下角的矩形的个数
            int lastX = 0;
            int lastY = j;
            for (int k = black.size() - 1; k >= -1; k--) {
                //此处哨兵单元设计很巧妙，让k==-1的时候自动启用哨兵单元
                Point p;
                if (k == -1) p = new Point(i, 0);
                else p = black.get(k);
                if (p.y > j) continue;
                if (p.x > i || p.x < lastX) continue;
                now += (i - lastX) * (lastY - p.y);
                lastX = p.x;
                lastY = p.y;
            }
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1] + now;
        }
    }
    return dp[n][m];
}

Main() {
    Scanner cin = new Scanner(System.in);
    n = cin.nextInt();
    m = cin.nextInt();
    dp = new long[n + 1][m + 1];
    black = new LinkedList<>();
    int k = cin.nextInt();
    while (k-- > 0) {
        int r = cin.nextInt(), c = cin.nextInt();
        black.add(new Point(r, c));
    }
    //对全部点进行排序（按照列从小到大进行排序）
    black.sort(Comparator.comparing(x -> x.y));
    for (int i = 0; i <= n; i++) dp[i][0] = 0;
    for (int i = 0; i <= m; i++) dp[0][i] = 0;
    long ans = solve();
    System.out.println(ans);
}

public static void main(String[] args) {
    new Main();
}
}
```

可以通过“缩点”法降低复杂度。

# 方法二：容斥原理
10个点有1024种情况，判断每种情况是否合理，然后统计个数即可。

复杂度之和k有关，跟n、m无关。

# 相似问题
codeforce上有一道规模更大的题，n和m为10000，k为20：
http://codeforces.com/gym/101350/problem/G

51nod
http://www.51nod.com/onlineJudge/questionCode.html#!problemId=1291
```cpp
#include<bits/stdc++.h>
using namespace std;
#define st first
#define nd second
#define rep(i, a, b) for(int i=(a); i<(b); i++)
#define sz(x) (int)x.size()
#define de(x) cout<< #x<<" = "<<x<<endl
#define dd(x) cout<< #x<<" = "<<x<<" "
typedef long long ll;
typedef pair<int, int> pii;
typedef vector<int> vi;

const int N = 666;
int n, m, top;
int u[N], sta[N];
ll c[N][N];
char s[N];

int main() {
    scanf("%d%d", &n, &m);
    for(int i = 1; i <= n; i++) {
        scanf("%s", s+1);
        for(int j = 1; j <= m; j++)
            u[j] = (s[j] == '1')? u[j]+1: 0;
        top = 0;
        sta[top++]=0;
        for(int j = 1; j <= m+1; j++) {
            while(u[sta[top-1]] > u[j]) {
                ++c[max(u[sta[top-2]], u[j])+1][j-sta[top-2]-1];//维护单调上升的栈, 看每个柱块向左和向右的最大延伸距离, 即为宽度
                --c[u[sta[top-1]]+1][j-sta[top-2]-1];            //枚举i为底边, 对高度范围为[max(u[sta[top-2]],u[j])+1, u[sta[top-1]]], 宽度为j-sta[top-2]-1的矩形加1
                --top;
            }
            while(top && u[sta[top-1]] == u[j]) --top;
            sta[top++] = j;
        }
    }
    for(int i = 2; i <= n; i++) for(int j = 1; j <= m; j++) c[i][j] += c[i-1][j]; //c1[i, j]: 高为i (n^2) 的连通块, 只统计最长宽度的。
    for(int i = 1; i <= n; i++) {
        for(int j = m-1; j; j--) c[i][j] += c[i][j+1];    //c2[i, j]: 高i宽 >= j的连通块的个数
        for(int j = m-1; j; j--) c[i][j] += c[i][j+1];  //c3[i, j]: 高i宽j的全1矩阵的个数
    }
    for(int i = 1; i <= n; i++) for(int j = 1; j <= m; j++) printf("%lld%c", c[i][j], " \n"[j == m]);
    return 0;
}
/*
3 3
011
110
110
如何得到c1? 暴力的话, c1是枚举n^2的上下边界, 优化后, 变成枚举直方图的最底边，快速统计各个不同的上顶边。这个可以通过单调栈+差分解决。求前缀和后就得到c1。
总的时间复杂度为O(nm).
c1
0 3 0
1 1 0
1 0 0
c2
3 3 0
2 1 0
1 0 0
c3
6 3 0
3 1 0
1 0 0

*/
```
leetcode
https://www.nowcoder.com/acm/contest/79/D
```cpp
#include<bits/stdc++.h>
using namespace std;
#define st first
#define nd second
typedef long long ll;
const int mod = 1e9+7;
pair<int, int> s[5111];
int main() {
    int n, m, c;
    scanf("%d%d%d", &n, &m, &c);
    for(int i = 1, x, y; i <= c; i++) scanf("%d%d", &x, &y), s[i] = {x, y};
    sort(s+1, s+c+1);
    long long ans = 0;
    for(int i = 1; i <= c; i++){
        int l = 0, r = m+1;
        for(int j = i-1; ~j; j--){                                                            //从下往上从左往右枚举之前的点
            ans += (r-s[i].nd)*(n-s[i].st+1LL)%mod*(s[j+1].st-s[j].st)%mod*(s[i].nd-l)%mod;    //上边界为s[j].st ~ s[j+1].st
            ans %= mod;
            if(s[j].nd > s[i].nd) r = min(r, s[j].nd);
            if(s[j].nd < s[i].nd) l = max(l, s[j].nd);
        }
    } 
    //n*m矩阵的子矩阵个数 = 左右两边界 * 上下两边界 
    long long cnt = (n*(n+1LL)/2%mod)*(m*(m+1LL)/2%mod)%mod;
    ans = (cnt-ans)%mod;
    if(ans < 0) ans += mod;
    printf("%lld\n", ans);
    return 0;
}
```

# 参考资料
https://www.cnblogs.com/dirge/p/10034268.html
        