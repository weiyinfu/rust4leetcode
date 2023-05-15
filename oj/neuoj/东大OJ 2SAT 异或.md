------------------
title = "东大OJ 2SAT 异或"
publishTime = "2014-08-01 10:50:00"
id = "5013897"
tags = [ "算法", "东大OJ",]

--------------

看了十年才懂懂了十年才会会了十年才会写写了十年才写完写完了十年才能改对

```cpp
#include<stdio.h>
#include<string.h>
struct res{
	int steps;
	int father;
};
int a[50001];
res findfather(int me){
	res r;
	r.steps = 0;
	while (a[me] != -1){ me = a[me]; r.steps++; }
	r.father = me;
	return r;
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	int n, m;
	int t;
	scanf(&quot;%d&quot;, &amp;t);
	int tt;
	for (tt = 1; tt &lt;= t; tt++){
		scanf(&quot;%d%d&quot;, &amp;n, &amp;m);
		memset(a, -1, sizeof(a));
		int i;
		int from, to;
		for (i = 0; i &lt; m; i++){
			scanf(&quot;%d%d&quot;, &amp;from, &amp;to);
			res rf = findfather(from);
			res rt = findfather(to);
			if (rf.father == rt.father)
			{
				if((rf.steps + rt.steps) % 2 == 0)
					break; 
				else continue;
			}
			if (rt.steps%2==0)a[rt.father] = from;
			else if (rf.steps % 2 == 0)a[rf.father] = to;
			else a[rf.father] = a[to];
		}
		if (i == m)printf(&quot;Test case #%d:\nNothing special.\n\n&quot;,tt);
		else printf(&quot;Test case #%d:\nSomething wrong!\n\n&quot;,tt);
		for (i++; i &lt; m; i++)scanf(&quot;%d%d&quot;, &amp;from, &amp;to);
	}
	return 0;
}
```
        