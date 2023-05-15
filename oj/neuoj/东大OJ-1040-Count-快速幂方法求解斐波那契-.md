------------------
title = "东大OJ-1040-Count-快速幂方法求解斐波那契-"
publishTime = "2014-07-30 02:36:00"
id = "5013906"
tags = [ "算法", "东大OJ",]

--------------

<div class="content">
<p lang="en-US" style="margin-bottom:0.17in; text-indent:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14.399999618530273px">&nbsp;</span></span></p>
<div class="content">
<p lang="en-US" style="margin-bottom:0.17in; text-indent:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">Many ACM team name may be very funny,such as &quot;Complier_Error&quot;,&quot;VVVVV&quot;.Oh,wait for a minute here.</span></span></p>
<p lang="en-US" style="margin-bottom:0.17in; text-indent:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">Is it &quot;W W&quot;&#43;&quot;V&quot;,or &quot;W&quot;&#43;&quot;V V V&quot;,or something others we can treat as?There are several ways we can treat this name &quot;VVVVV&quot; (5 'V's),as
 V V can be treat as a W.</span></span></p>
<p lang="en-US" style="margin-bottom:0.17in; text-indent:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">For 5 'V's,our have 8 ways.They are:</span></span></p>
<ol>
<li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">V V V V V</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">V W W
</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">W W V</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">V W V V</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">W V W</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">W V V V</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">V V W V</span></span></p>
</li><li>
<p lang="en-US" style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">V V V W</span></span></p>
</li></ol>
<p style="margin-bottom:0in; margin-top:0.19in"><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">The problem here is that for n 'V's,how many ways do we have to treat it?</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">Because
 the answer may be too large, you should output the answer module by </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">p</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">.</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">(If
 n is 0,then we have just one way.)</span></span></span></p>
<p></p>
</div>
<h2>输入</h2>
<div class="content">
<p style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">There are multiple test cases. The first line of the input contains an integer
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">M</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">, meaning the number of the test cases.</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><br>
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px">For each test cases, there are
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">two</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> integers n
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">and p
</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">in a single line.</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><br>
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px">You can assume that
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">0&lt;=n&lt;=2100000000</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">,
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">0&lt;p&lt;=2009</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">.</span></span></p>
<p></p>
</div>
<h2>输出</h2>
<div class="content">
<p style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">For each test case, output the answer with case number in a single line.</span></span></p>
<p></p>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">2
5 5
4 7</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">3
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata"></span><pre name="code" class="cpp">#include&lt;iostream&gt;
using namespace std;
struct m{int a[2][2]; };
m  mul(m a, m b,int p){
	m c;
	int i, j,k;
	for (i = 0; i &lt; 2;i++)
	for (j = 0; j &lt; 2; j++)
	{
		c.a[i][j] = 0;
		for (k = 0; k &lt; 2; k++)
			c.a[i][j] +=( (a.a[i][k] %p)* (b.a[j][k]%p))%p;
		c.a[i][j] %= p;
	}
	return c;
}
m go(m a, int n,int p){
	if (n == 1)return a;
	m b = go(a, n / 2, p);
	m c = mul(b, b, p);
	if (n % 2 == 1)c = mul(c, a, p);
	return c;
}
int main(){
	//freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	int t;
	cin &gt;&gt; t;
	m a = { 0, 1, 1, 1 };
	while (t-- &gt; 0){
		int n, p;
		cin &gt;&gt; n &gt;&gt; p;
		if (n == 0){ cout &lt;&lt; 1 &lt;&lt; endl; continue; }
		cout &lt;&lt; go(a, n, p).a[1][1]&lt;&lt;endl;
	}
	return 0;
}</pre><br><br></pre>
<p></p>
</div>
        