------------------
title = "东大OJ-Max Area"
publishTime = "2014-07-31 09:49:00"
id = "5013902"
tags = [ "算法", "东大OJ",]

--------------
1034: Max Area
时间限制: 1 Sec  内存限制: 128 MB
提交: 40  解决: 6

# 题目描述
又是这道题，请不要惊讶，也许你已经见过了，那就请你再来做一遍吧。这可是wolf最骄傲的题目哦。在笛卡尔坐标系正半轴（x>=0,y>=0）上有n个点，给出了这些点的横坐标和纵坐标，但麻烦的是这些点的坐标没有配对好，你的任务就是将这n个点的横坐标和纵坐标配对好，使得这n个点与x轴围成的面积最大。
# 输入
在数据的第一行有一个正整数m,表示有m组测试实例。接下来有m行，每行表示一组测试实例。每行的第一个数n，表示给出了n个点，接着给出了n个x坐标和y坐标。（给出的x轴的数据不会重复，y轴数据也不会重复）`（m<5000,1<n<50)`
# 输出
输出所计算的最大面积，结果保留两位小数，每组数据占一行。
# 样例输入
2
4 0 1 3 5 1 2 3 4
6 14 0 5 4 6 8 1 5 6 2 4 3
# 样例输出
15.00
59.00

```cpp
#include<stdio.h>
void sort(double *a, int from, int to){
	if (to <= from)return;
	int i = from, j = to;
	double k = a[from];
	while (1){
		while (a[j] > k)j--;
		if (j == i)break;
		a[i] = a[j];
		a[j] = k;
		i++;
		while (a[i] < k)i++;
		if (j == i)break;
		a[j] = a[i];
		a[i] = k;
		j--;
	}
	sort(a, from, i - 1);
	sort(a, i + 1, to);
}
int main()
{
	//freopen("in.txt", "r", stdin);
	int t;
	scanf("%d", &t);
	while (t-- > 0){
		int n;
		scanf("%d", &n);
		double x[5001];
		double  y[5001];
		double z[5001];
		int i;
		for (i = 0; i < n; i++)scanf("%lf", &x[i]);
		for (i = 0; i < n; i++)scanf("%lf", &y[i]);
		sort(x, 0, n - 1);
		for (i = 1; i < n - 1; i++)
			z[i] = x[i + 1] - x[i - 1];
		z[0] = x[1] - x[0];
		z[n - 1] = x[n - 1] - x[n - 2];
		sort(z, 0, n - 1);
		sort(y, 0, n - 1);
		double ans = 0;
		for (i = 0; i < n; i++)
			ans +=  z[i] *  y[i];
		printf("%.2lf\n", ans / 2);
	}
	return 0;
}
/*破东大OJ题里没说清楚,点是double类型,那个m是5000可能.
  若问这道题怎么做,
  第一关,走两步,列出式子;
  第二关,必须知道一个不等式:
     顺序>乱序>逆序
	 例如:a={1,2,3}b={4,5,6}
	 则1*4+2*5+3*6>乱序>1*6+2*5+1*4
	 */
```