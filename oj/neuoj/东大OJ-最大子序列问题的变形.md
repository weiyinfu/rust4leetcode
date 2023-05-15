------------------
title = "东大OJ-最大子序列问题的变形"
publishTime = "2015-01-31 13:51:00"
id = "5013893"
tags = [ "算法", "东大OJ",]

--------------

<center>
<h2>1302: 最大子序列</h2>
<span class="green">时间限制: </span>1 Sec&nbsp;&nbsp;<span class="green">内存限制: </span>128 MB<br>
<span class="green">提交: </span>224&nbsp;&nbsp;<span class="green">解决: </span>54<br>
[<a target="_blank" href="http://acm.neu.edu.cn/hustoj/submitpage.php?id=1302">提交</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/problemstatus.php?id=1302">状态</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/bbs.php?pid=1302">讨论版</a>]</center>
<h2>题目描述</h2>
<div class="content">
<p>给定一个N个整数组成的序列，整数有正有负，找出两段不重叠的连续子序列，使得它们中整数的和最大。两段子序列都可以为空。</p>
<p></p>
</div>
<h2>输入</h2>
<div class="content">
<div>多组输入，每组第一行为N，表示序列的长度；第二行为N个整数(-1000&lt;=n&lt;=1000)，表示输入序列。</div>
<div>0&lt;N&lt;=1,000,000</div>
<p></p>
</div>
<h2>输出</h2>
<div class="content">
<p>对于每组输入，输出一行，仅一个整数，表示最大的和。</p>
<p></p>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">9
185 -580 -889 701 964 -878 353 -761 608</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">2273</span></pre>
<pre class="content"><span class="sampledata">第一步，求出最大子序列M；M表示max
第二步，求出不与M相交的第二大子序列S；S表示second
第三步，求出M中的最小子序列L；L表示little
最后，分两种情况：M&#43;S或者是M一分为二；
若S&#43;L&lt;0，说明L太小了，M应该舍弃L，一分为二；
否则，M&#43;=S；
</span></pre>
<pre class="content"><span class="sampledata"></span><pre name="code" class="cpp">#include&lt;iostream&gt;
using namespace std;
int a[1000007];
int m[1000007];
int from, to;
int M, S, L;
int n;
void init(){
	int i;
	M = 0;
	m[0] = a[0];
	int f, t;
	f = t = 0;
	for (i = 1; i &lt; n; i++)
	{
		if (m[i - 1]&gt;0){
			m[i] = m[i - 1] + a[i];
			t++;
		}
		else{
			f = t = i;
			m[i] = a[i];
		}
		if (M &lt; m[i]){
			M = m[i];
			from = f;
			to = t;
		}
	}
}
void getS(){
	int i;
	int mm = 0;
	for (i = 0; i &lt; from; i++){
		if (m[i]&gt;mm)
			mm = m[i];
	}
	int now = a[to + 1];
	for (i = to + 2; i &lt; n; i++){
		if (now&gt;0){
			now += a[i];
		}
		else{
			now = a[i];
		}
		if (now &gt; mm)mm = now;
	}
	S = mm;
}
void getL(){
	int i;
	int ll = a[from];
	int now = a[from];
	for (i = from + 1; i &lt; to + 1; i++){
		if (now&lt;0){
			now += a[i];
		}
		else{
			now = a[i];
		}
		if (now &lt; ll)ll = now;
	}
	L = ll;
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	while (scanf(&quot;%d&quot;, &amp;n) != -1){
		int i;
		for (i = 0; i &lt; n; i++){
			scanf(&quot;%d&quot;, &amp;a[i]);
		}
		init();
		getS();
		getL();  
		if (S + L &lt; 0){
			M -= L;
		}
		else{
			M += S;
		}
		printf(&quot;%d\n&quot;, M);
	}
	return 0;
}
</pre><br></pre>
        