------------------
title = "hihocoder第238周：杨氏矩阵的个数"
publishTime = "2019-01-19 20:03:00"
id = "10293918"
tags = [ "算法",]

--------------

[题目链接](http://hihocoder.com/contest/hiho238/solution/1443145)

# 问题描述
给定一个N行M列的矩阵，往里面填入$`1-N\times M`$个数字，使得这个矩阵每行、每列都满足递增。问：有多少种填法？

# 问题分析
这个问题很难，如果能够直接想到，那就是天才了。
此问题中描述的矩阵就是杨氏矩阵的特例。杨氏矩阵又叫杨氏图表。
杨氏图表，它是这样一个二维表，满足条件：
(1)如果格子(i,j)没有元素，则它右边和上边的相邻格子也一定没有元素。
(2)如果格子(i,j)有元素a[i,j]，则它右边和上边的相邻格子要么没有元素，要么有元素且比a[i][j]大。

杨氏矩阵的计数公式为：
$$`count=\frac{n!}{\sum_{x \in Grids}{hook(x)}}`$$

其中$`hook(x)`$表示格子x下方、右方的空白格点数（不包括它自己）之和+1。

# 关键方法
由杨氏矩阵的计数公式可知，此问题是一道数学题。关键在于模除运算，这可以通过扩展欧几里得算法求逆元来实现。
```cpp
#include<stdio.h>
#include<iostream>
using namespace std;
typedef long long ll;
const int mod = 1e9 + 7;
ll gcd(ll a, ll b, ll &x, ll &y) {
	if (b == 0) {
		x = 1, y = 0;
		return a;
	}
	ll q = gcd(b, a%b, y, x);
	y -= a / b * x;
	return q;
}
ll reverse(int v) {
	ll x, y;
	ll g = gcd(v, mod, x, y);
	return x;
}
int main() {
	int n, m;
	cin >> n >> m;
	ll s = 1;
	for (int i = 1; i <= n * m; i++) {
		s *= i;
		s %= mod;
	}
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < m; j++) {
			int hook = n - i + m - j-1;
			int r = reverse(hook);
			s *= r;
			s %= mod;
		}
	}
	s = (s + mod) % mod;
	cout << s << endl;
	return 0;
}
```
        