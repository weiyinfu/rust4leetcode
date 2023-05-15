------------------
title = "东大OJ-1430-PrimeNumbers"
publishTime = "2014-07-29 08:18:00"
id = "5013910"
tags = [ "算法", "东大OJ",]

--------------

<h2>题目描述</h2>
<div class="content">
<div></div>
<div>I'll give you a number , please tell me how many different prime factors in this number.</div>
</div>
<h2>输入</h2>
<div class="content">
<p>There is multiple test cases , in each test case there is only one line contains a number N(2&lt;=N&lt;=100000). Process to the end of file.</p>
<p></p>
</div>
<h2>输出</h2>
<div class="content">
<p>For each test case , output one line contains one number , indicating different prime factor in the number N.</p>
<p></p>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">12
5
30
</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">2
1
3
</span></pre>
<h2>提示</h2>
<div class="content">
<p></p>
<div>12 = 2 * 2 * 3</div>
<br>
<div>5 = 5</div>
<br>
<div>30 = 2 * 3 * 5&nbsp;</div>
<div><br>
</div>
<div><br>
</div>
<div><pre name="code" class="cpp">#include&lt;iostream&gt;
using namespace std;
int a[100001];
void go(){
	memset(a, 0, sizeof(a));
	int i,j;
	for (i = 2; i &lt; 100001;i++)
	if (a[i] == 0){
		for (j = i; j &lt; 100001; j+=i)a[j]++;
	}
}
int main(){
	//freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	int x;
	go();
	while (cin &gt;&gt; x)cout &lt;&lt; a[x] &lt;&lt; endl;
	return 0;
}</pre><br>
<br>
</div>
</div>
        