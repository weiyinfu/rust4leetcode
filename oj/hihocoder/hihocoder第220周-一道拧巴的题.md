------------------
title = "hihocoder第220周-一道拧巴的题"
publishTime = "2018-09-19 12:59:00"
id = "9677579"
tags = [ "算法", "hihocoder",]

--------------

# 一、220周
[题目链接](http://hihocoder.com/contest/hiho220/rank)
# 问题描述
键盘上有N个数字按键，每个按键只能按一次，每次可以按下多个键，请输出所有可能的按键情况。
输入一个整数N（N在1~8之间），输出全部的按键可能。例如：输入3，输出为
```plain
1-2-3
1-23
1-3-2
12-3
123
13-2
2-1-3
2-13
2-3-1
23-1
3-1-2
3-12
3-2-1
```
输出按照字符串大小从小到大输出。

# 思路
对于给定数组a，a中存放着备用数字，从a中分别取1，2，3，...len(a)个元素组成一组group。然后a-group=b，将b数组递归向后传递。

Java里面的迭代器比较难受，写出来的代码也拧巴，这点非常考验代码能力，是一道非常不错的面试题。

Python里面的生成器更简便。

# 代码
```java
import sun.reflect.generics.tree.Tree;

import java.util.*;
import java.util.stream.Collectors;

public class Main {
TreeSet<String> a = new TreeSet<>();

class SelectCount implements Iterator<String> {
    int[] a;
    int cnt;
    int[] p;
    boolean over = true;

    SelectCount(int[] a, int cnt) {
        this.a = a;
        this.cnt = cnt;
        p = new int[cnt];
        for (int i = 0; i < cnt; i++) {
            p[i] = i;
        }
    }

    @Override
    public boolean hasNext() {
        return over;
    }

    void move() {
        for (int i = cnt - 1; i >= 0; i--) {
            if (p[i] != a.length - cnt + i) {
                p[i]++;
                for (int j = i + 1; j < cnt; j++) {
                    p[j] = p[j - 1] + 1;
                }
                return;
            }
        }
        over = false;
    }

    @Override
    public String next() {
        String ans = Arrays.stream(p).map(x -> a[x]).mapToObj(x -> x + "").collect(Collectors.joining());
        move();
        return ans;
    }
}

void go(int[] a, LinkedList<String> l) {
//    System.out.println(Arrays.stream(a).mapToObj(x -> x + "").collect(Collectors.joining()));
    if (a.length == 0) {
        this.a.add(l.stream().collect(Collectors.joining("-")));
        return;
    }
    for (int i = 1; i <= a.length; i++) {
        SelectCount sel = new SelectCount(a, i);
        while (sel.hasNext()) {
            String s = sel.next();
            int[] b = new int[a.length - i];
            int bi = 0;
            //a数组减去b数组
            for (int j = 0; j < a.length; j++) {
                if (!s.contains(a[j] + "")) {
                    b[bi++] = a[j];
                }
            }
            l.add(s);
            go(b, l);
            l.removeLast();
        }
    }
}

Main() {
    Scanner cin = new Scanner(System.in);
    int N = cin.nextInt();
    int[] a = new int[N];
    for (int i = 1; i <= N; i++) {
        a[i - 1] = i;
    }
    go(a, new LinkedList<>());
    for (String i : this.a) {
        System.out.println(i);
    }
}

public static void main(String[] args) {
    new Main();
}
}
```
# 二、221周
[题目链接](http://hihocoder.com/contest/hiho221/problems)
还是上面那样的题干，问题变成一道组合计数题，问一共有多少种按法。

枚举第一次按下的情况，剩下的情况递归解决，递归时可以利用DP加速，不加速肯定超时。计算组合数时，必须使用杨辉三角，否则太慢。使用编程语言时，必须用C/C++，连Java都会超时，这个超时当然是运行效率了。

```cpp
#include<stdio.h>
#include<iostream> 
#include<stdlib.h>
#include<string.h>
using namespace std; 
int mod = 1000000007;
typedef long long ll;
ll c[1007][1007];
void init() {
	memset(c, 0, sizeof(c));
	for (int i = 0; i < 1007; i++)c[i][0] = 1;
	c[0][0] = 1;
	for (int i = 1; i < 1007; i++) {
		for (int j = 1; j <= i; j++) {
			c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
			c[i][j] %= mod;
		}
	} 
}
ll dp[1007];
ll solve(int N) {
	if (N == 1 || N == 0)return 1;
	if (dp[N] != -1)return dp[N];
	ll s = 0;
	for (int i = 1; i <= N; i++) {
		s += c[N][i]%mod * solve(N - i)%mod;
		s %= mod;
	}
	dp[N] = s;
	return s;
}
int main() {
	int N;
	cin >> N;
	init(); 
	memset(dp, -1, sizeof(dp));
	cout << solve(N);
	return 0;
}
```
        