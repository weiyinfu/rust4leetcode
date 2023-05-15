------------------
title = "东大oj-1591 Circle of friends"
publishTime = "2015-06-10 06:03:00"
id = "5013882"
tags = [ "算法", "东大OJ",]

--------------

# 题目描述
Nowadays, "Circle of Friends" is a very popular social networking platform in WeChat. We can share our life to friends through it or get other's situation.
Similarly, in real life, there is also a circle of friends, friends would often get together communicating and playing to maintain friendship. And when you have difficulties, friends will generally come to help and ask nothing for return.
However, the friendship above is true friend relationship while sometimes you may regard someone as your friend but he doesn't agree.In this way when you ask him for help, he often asks you for a meal, and then he will help you.
If two people think they are friends mutually,they will become true friend,then once one of them has a problem or makes a query, the other one will offer help for free.What's more,if one relationship is similar to “A regards B as friend, B regards C as friend and C regards A as friend”,they will make a friends circle and become true friends too with each other. Besides, people will not ask those who they don’t regard as friends for help. If one person received a question and he can not solve it, he will ask his friends for help. 
Now, Nias encounters a big problem, and he wants to look for Selina's help. Given the network of friends, please return the minimum number of meals Nias must offer. Of course Nias is lavish enough, so he will pay for all the meals in the network of friends.

# 输入
The first line of input contains an integer T, indicating the number of test cases (T<=30).
For each test case, the first line contains two integers, N and M represent the number of friends in the Nias’s network and the number of relationships in that network. N and M are less than 100000 and you can assume that 0 is Nias and n-1 is Selina.
Next M lines each contains two integers A and B, represent a relationship that A regards B as his friend, A and B are between 0 and n-1.

# 输出
 For each test case, please output the minimum number of meals Nias need to offer; if Nias can’t get Selina’s help, please output -1.

# 样例
样例输入
```
3
4 4
0 1
1 2
2 1
2 3

3 3
0 1
1 2
2 1

3 1
0 1
```
样例输出
```
2
1
-1
```

# 题解
首先,深度优先求有向图强连通分量(Tarjan)算法
其次,构图,所构造出来的新图是一个拓扑图
最后,spfa求最短路径.
知识点:
spfa适用于稀疏图最短路径,dijstra适合稠密图最短路.  


```cpp
#include<iostream>
#include<stdio.h>
#include<string.h>
#include<algorithm>
using namespace std;
const int maxn = 1e5 + 7;
int N, M;
struct Edge{
	int to, next;
}e[maxn*2];
int ei,g[maxn];
void push_back(int f, int t){ 
	e[++ei].to = t;
	e[ei].next = g[f];
	g[f] = ei;
}
int ti, pre[maxn], low[maxn],num[maxn],sccNo;
int a[maxn];
struct Stack{
	int a[maxn];
	int i;
	void init(){ i = 0; }
	void push(int x){a[i++] = x;}
	int pop(){return a[--i];}
}sta;
void dfs(int now){
	pre[now] = low[now] = ++ti;
	sta.push(now);
	for (int i = g[now]; i; i = e[i].next){
		int t = e[i].to;
		if (pre[t] == 0)dfs(t), low[now] = min(low[now], low[t]);
		else if (num[t] == 0)low[now] = min(low[now], pre[t]);
	}
	if (pre[now] == low[now]){
		++sccNo; 
		while (true){
			int x = sta.pop();
			num[x] = sccNo;
			if (x == now)break;
		}
	}
}
void newGraph(){
	memset(a, 0, sizeof(int)*(1+sccNo));
	for (int i = 0; i < N; i++){
		for (int j = g[i]; j; j = e[j].next){
			int t = e[j].to;
			if (num[i] ^ num[t]){
				e[++ei].next = a[num[i]];
				e[ei].to = num[t];
				a[num[i]] = ei;
			}
		}
	}
}
struct Q{
	int a[maxn], head, rear;
	bool has[maxn];
	void init(){ head = rear = 0; memset(has, 0, sizeof(int)*(sccNo+1)); }
	void enq(int x){ a[rear] = x; rear = (rear + 1) % maxn; has[x] = 1; }
	int deq(){ int ans = a[head]; head = (head + 1) % maxn; has[ans] = 0; return ans; }
}q;
int dis[maxn]; 
void spfa(){
	q.init();
	q.enq(num[0]);
	memset(dis, 0x34, sizeof(int)*(1 + sccNo));
	dis[num[0]] = 0;
	while (q.head^q.rear){
		int now = q.deq();
		for (int i = a[now]; i; i = e[i].next){
			int t = e[i].to;
			if (dis[t] > dis[now] + 1){
				dis[t] = dis[now] + 1;
				if (q.has[t]== false){
					q.enq(t);
				}
			}
		}
	}
	if (dis[num[N - 1]] == 0x34343434)printf("-1\n");
	else printf("%d\n", dis[num[N - 1]]);
}
int main(){
	freopen("in.txt", "r", stdin);
	int T; scanf("%d", &T);
	while (T--){
		scanf("%d%d", &N, &M);
		ei = 0, memset(g, 0, sizeof(int)*N);
		while (M--){
			int x, y; scanf("%d%d", &x, &y);
			push_back(x, y);
		}
		ti = 0, memset(pre, 0, sizeof(int)*N), memset(num, 0, sizeof(int)*N);
		sta.init(), sccNo = 0;
		for (int i = 0; i < N; i++)if (pre[i] == 0)dfs(i);
		newGraph(); 
		spfa();
	}
	return 0;
}
```
        