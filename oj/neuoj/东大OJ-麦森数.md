------------------
title = "东大OJ-麦森数"
publishTime = "2014-07-31 13:48:00"
id = "5013900"
tags = [ "算法", "东大OJ",]

--------------

<center>
<h2>1064: 麦森数</h2>
<span class="green">时间限制: </span>1 Sec&nbsp;&nbsp;<span class="green">内存限制: </span>128 MB<br>
<span class="green">提交: </span>52&nbsp;&nbsp;<span class="green">解决: </span>9<br>
[<a target="_blank" href="http://acm.neu.edu.cn/hustoj/submitpage.php?id=1064">提交</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/problemstatus.php?id=1064">状态</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/bbs.php?pid=1064">讨论版</a>]</center>
<h2>题目描述</h2>
<div class="content">
<div>形如2P-1的素数称为麦森数，这时P一定也是个素数。但反过来不一定，即如果P是个素数，2P-1不一定也是素数。到1998年底，人们已找到了37个麦森数。最大的一个是P=3021377，它有909526位。麦森数有许多重要应用，它与完全数密切相关。</div>
<div>任务：从文件中输入P（1000&lt;P&lt;3100000），计算2P-1的位数和最后500位数字（用十进制高精度数表示）</div>
</div>
<h2>输入</h2>
<div class="content">
<p>文件中只包含一个整数P（1000&lt;P&lt;3100000）</p>
</div>
<h2>输出</h2>
<div class="content">
<div>第一行：十进制高精度数2P-1的位数。</div>
<div>第2行：十进制高精度数2P-1的最后500位数字。（不足500位时高位补0）</div>
<div>不必验证2P-1与P是否为素数。</div>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">1279</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">386
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010407932194664399081925240327364085538615262247266704805319112350403608059673360298012239441732324184842421613954281007791383566248323464908139906605677320762924129509389220345773183349661583550472959420547689811211693677147548478866962501384438260291732348885311160828538416585028255604666224831890918801847068222203140521026698435488732958028878050869736186900714720710555703168729087</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata"></span><pre name="code" class="cpp">#include&lt;stdio.h&gt;
#include&lt;string.h&gt;
#include&lt;math.h&gt;
/*
   第一层难点要知道用对数求位数
   第二层难点要进行大整数运算
*/
struct num { char a[500]; int size; };
void shl(num &amp;n,int k){
	int i;
	i = n.size - 1;
	if (i + k &gt;= 500)i = 500 - 1 - k;
	for (; i &gt;= 0; i--)
		n.a[i + k] = n.a[i];
	n.size += k;
	if (n.size &gt; 500)n.size = 500;
	for (i = 0; i &lt; k; i++)n.a[i] = 0;
}
num add(num a, num b){
	num c;
	memset(&amp;c, 0, sizeof(c));
	int i = 0;
	if (a.size &gt; b.size)c.size = a.size;
	else c.size = b.size;
	for (i = 0; i &lt; c.size; i++)
	{
		c.a[i] += a.a[i] + b.a[i];
		c.a[i + 1] += c.a[i] / 10;
		c.a[i] %= 10;
	}
	if (c.a[i] != 0)c.size++;
	if (c.size&gt;500)c.size = 500;
	return c;
}
num multiply(num a, int b){
	num c;
	memset(&amp;c, 0, sizeof(c));
	if (b == 0)return c;
	if (b == 1)return a;
	c.size = a.size;
	int i;
	for (i = 0; i &lt; a.size; i++){
		c.a[i] += a.a[i] * b;
		c.a[i + 1] += c.a[i] / 10;
		c.a[i] %= 10;
	}
	if (c.a[i] != 0)c.size++;
	if (c.size&gt;500)c.size = 500;
	return c;
}
num mul(num a, num b){
	num c,t;
	memset(&amp;c, 0, sizeof(c));
	int i;
	for (i = 0; i &lt; b.size; i++){
		memcpy(&amp;t ,&amp; multiply(a, b.a[i]),sizeof(t));
		shl(t, i);
		memcpy(&amp;c ,&amp;add(c, t),sizeof(c));
	}
	return c;
}
num pow(num a, int k){
	if (k == 1)return a;
	num t;
	memcpy(&amp;t ,&amp; pow(a,k / 2),sizeof(t));
	if (k % 2 == 1)return mul(mul(t, t), a);
	else return mul(t, t);
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	int p;
	scanf(&quot;%d&quot;, &amp;p);
	num a;
	memset(&amp;a, 0, sizeof(a));
	a.size = 1;
	a.a[0] = 2;
	memcpy(&amp;a,&amp;pow(a,p),sizeof(a));
	int digit = log10((double)2)*p;
	printf(&quot;%d\n&quot;, digit + 1);
	int i;
	for (i = 0; i &lt; 500; i++)
	if (a.a[i] == 0)a.a[i] = 9;
	else break;
	a.a[i]--;
	for (i = 0; i &lt; 500;i++)
		printf(&quot;%d&quot;, a.a[500-1-i]);
	return 0;
}</pre><br><br></pre>
        