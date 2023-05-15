------------------
title = "hihocoder第233周"
publishTime = "2018-12-15 18:13:00"
id = "10125574"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho233/problem/1)

# 题目描述
给定一个数组a[N],N小于1e5。把数组划分成若干个片段，每个片段的和都不为0，问有多少种划分方法？

# 方法描述
定义f(i)表示0~i共有多少种划分方式，则$f(j)=\sum_{i\in[0,j) and sum(a[i+1:j]) \ne 0} f(i)$
相当于统计$f(j)=\sum_{i \in [0,j)} f(i)-\sum_{i \in [0,j) and sum(a[i+1:j])==0} f(i)$。对于此式第二项可以使用map记录下来，满足sum(a[i+1:j])的那些i，必定满足prefix[i]==prefix[j]，prefix[i]表示前缀和，即0~i之间全部元素之和。

```cpp
#include<iostream>
#include<stdio.h>
#include<map> 
using namespace std; 
typedef long long ll;
const int maxn = 1e9 + 7;
const int maxcount = 1e5 + 3;
const int maxvalue = 103; 
int n;
int a[maxcount];
int pre[maxcount];
map<int, int>ma; 
int main() {
	freopen("in.txt", "r", stdin);
	cin >> n;
	for (int i = 0; i < n; i++)scanf("%d", a + i+1); 
	pre[0]=a[0] = 0;
	ma[0] = 1;
	for (int i = 1; i <= n; i++)pre[i] = pre[i - 1] + a[i];  
	ll s = 1;
	ll now = 0;
	for (int i=1; i <= n; i++) {  
		now = (s- ma[pre[i]]+maxn)%maxn;  
		s = (s + now) % maxn; 
		if (ma.count(pre[i]) == 0)ma[pre[i]] = 0;
		ma[pre[i]] = (ma[pre[i]]+now)%maxn;
	} 
	cout << now<< endl;
	return 0;
}
```
        