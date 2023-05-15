------------------
title = "东大OJ-快速排序"
publishTime = "2014-07-31 23:20:00"
id = "5013899"
tags = [ "算法", "东大OJ",]

--------------

<center>
<h2>1236: Simple Sort</h2>
<span class="green">时间限制: </span>1 Sec&nbsp;&nbsp;<span class="green">内存限制: </span>128 MB<br>
<span class="green">提交: </span>195&nbsp;&nbsp;<span class="green">解决: </span>53<br>
[<a target="_blank" href="http://acm.neu.edu.cn/hustoj/submitpage.php?id=1236">提交</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/problemstatus.php?id=1236">状态</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/bbs.php?pid=1236">讨论版</a>]</center>
<h2>题目描述</h2>
<div class="content">
<div><span style="font-size:14px"><span style="font-family:Arial">&nbsp; &nbsp; &nbsp;You are given n two-dimension points randomly. Now you are asked to sort them by the following rule. For example , there are two points ,point A(x1,y1) and point B(x2,y2), to be compared.
 We define point A is less than point B if x1&lt;x2. When x1 equals to x2, we define point A is less than point B if y1&lt;y2. If x1 equals to x2 and y1 equals to y2, we say point A and point B are equal.</span></span></div>
<div><span style="font-size:14px"><span style="font-family:Arial">&nbsp; &nbsp; &nbsp;Now you need to sort the points in non-descending order according the rules.</span></span></div>
<p></p>
</div>
<h2>输入</h2>
<div class="content">
<div><span style="font-size:14px"><span style="font-family:Arial">&nbsp; &nbsp; There are serval test cases.</span></span></div>
<div><span style="font-size:14px"><span style="font-family:Arial">&nbsp; &nbsp; The first line contians a integer t which deticates the number of test cases.</span></span></div>
<div><span style="font-size:14px"><span style="font-family:Arial">&nbsp; &nbsp; For each test case, the first line contians a integer n deticating the number of points in the test case. The next n lines contain two integers per line which are the positions of n points.
 (0&lt;t&lt;=100, 0&lt;n&lt;=10000, -1000&lt;=x&lt;=1000, -1000&lt;=y&lt;=1000)</span></span></div>
<p></p>
</div>
<h2>输出</h2>
<div class="content">
<p><span style="font-size:14px"><span style="font-family:Arial">&nbsp; &nbsp; For each test case, the first line print &quot;Test case x:&quot; in which number x is the test case number starting from 1. There are n lines following. Print out the result of the sort.</span></span></p>
<p></p>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">3
3
10 2
5 4
3 9
3
7 8
8 4
7 5
1
4 4
</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">Test case 1:
3 9
5 4
10 2
Test case 2:
7 5
7 8
8 4
Test case 3:
4 4</span><pre name="code" class="cpp">#include&lt;stdio.h&gt;
struct point{ int x, y; };
point  a[10000];
void copy(point &amp;k, point&amp; i){
	k.x = i.x;
	k.y = i.y;
}
bool less(point i, point j){
	if (i.x &lt; j.x)return true;
	if (i.x==j.x&amp;&amp;i.y &lt; j.y)return true;
	return false;
}
bool big(point i, point j){
	if (i.x&gt;j.x)return true;
	if (i.x==j.x&amp;&amp;i.y&gt;j.y)return true;
	return false;
}
void sort(int from, int to){
	if (from &gt;= to)return;
	int i = from, j = to;
	point k;
	copy(k, a[from]);
	while (1){
		while (big(a[j], k))j--;
		if (j == i)break;
		copy(a[i], a[j]);
		copy(a[j], k);
		i++;
		while (less(a[i], k))i++;
		if (j == i)break;
		copy(a[j], a[i]);
		copy(a[i], k);
		j--;
	}
	sort(from, i - 1);
	sort(i + 1, to);
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	int t;
	scanf(&quot;%d&quot;, &amp;t);
	int tt;
	for(tt=1;tt&lt;=t;tt++){
		int n;
		scanf(&quot;%d&quot;, &amp;n);
		int i;
		for (i = 0; i &lt; n; i++)scanf(&quot;%d%d&quot;,&amp;a[i].x,&amp;a[i].y);
		sort(0, n - 1);
		printf(&quot;Test case %d:\n&quot;, tt);
		for (i = 0; i &lt; n; i++)
			printf(&quot;%d %d\n&quot;, a[i].x, a[i].y);
	}
	return 0;
}</pre><br></pre>
        