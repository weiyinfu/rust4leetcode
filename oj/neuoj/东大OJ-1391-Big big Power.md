------------------
title = "东大OJ-1391-Big big Power"
publishTime = "2014-07-29 13:34:00"
id = "5013909"
tags = [ "算法", "东大OJ",]

--------------

<h2>题目描述</h2>
<div class="content">
<p><span style="font-size:undefined">Calculate the power num a^(b^c) mod 1e9&#43;7</span></p>
<p></p>
</div>
<h2>输入</h2>
<div class="content">
<p><span style="font-size:undefined">Multiple test cases,each case has three integers a,b and c . a,b,c is less than 1e9;</span></p>
<p></p>
</div>
<h2>输出</h2>
<div class="content">
<p><span style="font-size:undefined">Output the answer in each line</span></p>
<p></p>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">2 3 2</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">512</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata"><strong><span style="font-family:KaiTi_GB2312; font-size:32px">    需要用到费马小定理.如果p是质数,并且a不等于1且a不等于p,那么   a^(p-1)%p====1.</span></strong></span></pre>
<pre class="content"><span class="sampledata"><strong><span style="font-family:KaiTi_GB2312; font-size:32px">所以,a^1000000006   %  1000000007=1.</span></strong></span></pre>
<pre class="content">
</pre>
<pre class="content"><span class="sampledata"><strong><span style="font-family:KaiTi_GB2312; font-size:32px">a^b^c%1000000007=a^(b^c % 1000000006) % 1000000007.</span></strong></span></pre>
<pre class="content"><span style="font-family:KaiTi_GB2312; font-size:32px"><strong>这是第一关.用费马小定理将问题化简一下.</strong></span></pre>
<pre class="content"><span style="font-family:KaiTi_GB2312; font-size:32px"><strong>第二关是快速幂,用递归实现特别清晰.</strong></span></pre>
<pre class="content"><span style="font-family:KaiTi_GB2312; font-size:32px"><strong>
</strong></span></pre>
<pre class="content"><span style="font-family:KaiTi_GB2312; font-size:32px"><strong>九指神丐洪七公曾经告诉郭靖:遇到敌人不要把注意力放在敌人出什么招数上,你只需要把这降龙十八掌一遍一遍的使将出来,必可立于不败之地.</strong></span></pre>
<pre class="content"><span style="font-family:KaiTi_GB2312; font-size:32px"><strong>管他是否多余,多用long long int.各种取余.</strong></span></pre>
<pre class="content"><span style="font-family:KaiTi_GB2312; font-size:32px"><strong>为了安全,可以适当的冗余.尤其是现在的大程序.</strong></span></pre>
<pre class="content">
</pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata"></span><pre code_snippet_id="438144" snippet_file_name="blog_20140729_1_5385012"  name="code" class="cpp">#include&lt;iostream&gt;
using namespace std;
long long int go(long long int a, long long int b, long long int c){
	if (b == 0)return 1;
	long long int t = go(a, b / 2, c);
	long long int p = (t*t) % c;
	if (b % 2 == 0)return p;
	return (p*a) % c;
}
int main(){
	//freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	long long int a, b, c;
	while (cin &gt;&gt; a &gt;&gt; b &gt;&gt; c)
		cout &lt;&lt; go(a, go(b, c, 1000000006), 1000000007)&lt;&lt;endl;
	return 0;
}</pre><br><br></pre>
        