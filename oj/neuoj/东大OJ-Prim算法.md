------------------
title = "东大OJ-Prim算法"
publishTime = "2014-08-01 21:08:00"
id = "5013896"
tags = [ "算法", "东大OJ",]

--------------
1222: Sweep the snow
时间限制: 1 Sec  内存限制: 128 MB
提交: 28  解决: 18

# 题目描述
After the big big snow yesterday, NEU became a beautiful silver world.In the morning, poor michaelalan got a short message which told him to sweep the whole school snow and make the traffic unblocked. As you see, it needs so much time to do by one person.There are some points need clear,and these points must connected together(eg:If there are A point and B point, then sweep a line between A and B is enough). So he wants to know the minimum length of snow he needs to sweep to ensure the traffic unblocked (the width of snow is ignored).

# 输入
Multiple inputs, process until the EOF.

First line there is a integer n (2<=n<=1000) which means the count of points needs clear.then following n lines with each line two double numbers indicate the x-coordinate and the y-coordinate of the point.

# 输出
The minimum length of snow need swept(reserve one decimal, %.1lf).

# 样例输入
30.0 0.01.0 1.01.0 0.041.0 0.00.0 0.00.0 1.01.0 1.080.0 0.02.5 3.51.1 1.19.0 10.03.1 1.35.5 3.50.0 3.03.0 0.0

# 样例输出
2.03.019.7

```cpp
#include<stdio.h>
#include<string.h>
#include<float.h>
#include<math.h>
struct point{ double x, y; };
point a[1000];
double dis[1000];
int n;
double distance(int i, int j){
	double x = a[i].x - a[j].x;
	double y = a[i].y - a[j].y;
	return sqrt(x*x + y*y);
}
double prim(){
	int i, next=n,now=0;
	double cost = 0;
	for (i = 1; i <= n; i++)dis[i] = DBL_MAX;
	dis[0] = 0;
	while (1){
		cost += dis[now];
		dis[now] = -1;
		for (i = 0; i < n; i++){
			if (dis[i] == -1)continue;
			double temp = distance(now, i);
			if (temp < dis[i])dis[i] = temp;
			if (dis[i] < dis[next])next = i;
		}
		if (next == n)return cost;
		now = next;
		next = n;
	}
}
int main(){
	freopen("in.txt", "r", stdin);
	while (scanf("%d", &n)!=-1){
		int i;
		for (i = 0; i < n; i++)
			scanf("%lf%lf", &a[i].x, &a[i].y);
		printf("%.1lf\n",prim());
	}
	return 0;
}
```