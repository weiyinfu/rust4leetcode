------------------
title = "hihocoder第237周：三等分带权树"
publishTime = "2019-01-19 17:52:00"
id = "10293892"
tags = [ "算法",]

--------------

[题目链接](http://hihocoder.com/problemset/problem/1479)

# 问题描述
给定一棵树，树中每个结点权值为[-100,100]之间的整数。树中包含结点总数不超过1e5。任选两个非根节点A、B，将这两个结点与其父节点断开，可以得到三棵子树。现要求三棵子树的权值之和相等，问A、B有多少种选择方法。

输入
```
T：样例种数
N：树中结点个数
v1 father1
v2 father2
```
# 问题分析
此问题是一道树形DP。
如何才能将一棵树划分成三棵权值之和相等的子树？有两种情况：
* 若结点x和结点y没有血缘关系（不是祖孙关系），则x和y的权值之和都是s（s为整棵树的权值之和的三分之一），x和y把这棵树截为三段。
* 若结点x和结点y有血缘关系，不妨设x是y的祖先，则x的权值之和为2s，y的权值之和为s。x和y把这棵树截为三段。

#最精简的代码
```cpp
#include<iostream>
#include<stdio.h> 
#include<iostream> 
using namespace std;
const int maxn = 1e5 + 7;
typedef  long long ll;
struct Node {
	int v;
	int s; 
	int son;
}a[maxn];
int nex[maxn];
int root;
int per;
int n;
ll ans;
int perCount = 0;
int go(int nodeId) {
	int temp = perCount;
	a[nodeId].s = a[nodeId].v;
	for (int i = a[nodeId].son; i != -1; i = nex[i]) {
		a[nodeId].s += go(i);
	}
	if (nodeId != root) {
		if (a[nodeId].s == per * 2) {
			ans += perCount - temp;
		}
		if (a[nodeId].s == per) {
			ans += temp;
			perCount++;
		}
	} 
	return a[nodeId].s;
}
void push(int parent, int son) {
	int temp = a[parent].son;
	nex[son] = temp;
	a[parent].son = son;
} 
int main() { 
	int T;
	cin >> T;
	while (T-- > 0) { 
		cin >> n;
		for (int i = 0; i <= n; i++) {
			a[i].son = -1;
			nex[i] = -1;
		}
		int s = 0;
		for (int i = 0; i < n; i++) {
			int father;
			cin >> a[i].v >> father;
			s += a[i].v;
			father--;
			if (father == -1) {
				root = i;
			}
			else {
				push(father, i);
			}
		}
		
		if (s % 3 == 0) {
			per = s / 3;
			ans = 0;
			perCount = 0;
			go(root); 
			cout << ans << endl;
		}
		else {
			cout << 0 << endl;
		} 
	}
	return 0;
}
```

# 复杂但是直观的代码
```cpp
#include<iostream>
#include<stdio.h>
#include<stdlib.h>
using namespace std;
const int maxn = 1e5 + 7;
typedef long long ll;
int n;
int per;//每个分支应该等于的数值
struct Node {
	int v;//结点权重
	int s;//结点所代表的子树的权重之和
	int sonPerCount;//子树中权值之和为per的结点个数（包括自身）
	int son;//儿子结点，链表第一个结点
	int next;//下一个兄弟结点
}a[maxn];
void push(int father, int son) {
	int temp = a[father].son;
	a[son].next = temp;
	a[father].son = son;
}
int root;
void init(int nodeId) {
	a[nodeId].s = a[nodeId].v;
	for (int i = a[nodeId].son; ~i; i = a[i].next) {
		init(i);
		a[nodeId].s += a[i].s;
		a[nodeId].sonPerCount += a[i].sonPerCount;
	}
	if (a[nodeId].s == per) {
		a[nodeId].sonPerCount++;
	}
}
ll count1 = 0, count2 = 0;//count1表示我的值为per时，count2表示我的值为2per时
int totalPer = 0;
int cnt = 0;
ll ans = 0;
void go(int nodeId) {
	if (a[nodeId].s == per) {
		if (nodeId != root) {
			count1 += totalPer - cnt - a[nodeId].sonPerCount;
		}
		cnt++;
	}
	//此处不能有else，因为当per=0时，子树中的总和为per的结点也会生效
	if (a[nodeId].s == per * 2) {
		if (nodeId != root) {
			count2 += a[nodeId].sonPerCount;
			if (per == 0)count2--;//因为sonPerCount包括结点自身，所以需要先去掉结点
		}
	}
	for (int i = a[nodeId].son; ~i; i = a[i].next) {
		go(i);
	}
	if (a[nodeId].s == per)cnt--;
}
int main() {
	freopen("in.txt", "r", stdin);
	int T; cin >> T;
	while (T--) {
		cin >> n;
		int s = 0;
		for (int i = 1; i <= n; i++) {
			a[i].sonPerCount = 0;
			a[i].son = -1;
			a[i].next = -1;
		}
		for (int i = 1; i <= n; i++) {
			int father;
			cin >> a[i].v >> father;
			if (father == 0) {
				root = i;
			}
			else {
				push(father, i);
			}
			s += a[i].v;
		}
		if (s % 3 == 0) {
			per = s / 3;
			init(root);
			count1 = count2 = 0;
			totalPer = 0;
			for (int i = 1; i <= n; i++) {
				if (a[i].s == per)totalPer++;
			}
			go(root);
			ans = count1 / 2 + count2;
			cout << ans << endl;
		}
		else { cout << 0 << endl; }
	}
	return 0;
}

```
# 注意事项
* 如果用Java写，此题会超时。因为输入量比较大
* 如果数据充分一些，1e5的复杂度有可能爆栈，所以需要使用栈的方式来遍历树（也可以先对树进行线索化）。
        