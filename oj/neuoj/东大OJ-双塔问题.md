------------------
title = "东大OJ-双塔问题"
publishTime = "2014-08-01 23:55:00"
id = "5013895"
tags = [ "算法", "东大OJ",]

--------------

<center>
<h2>1212: VIJOS-P1037</h2>
<span class="green">时间限制: </span>0 Sec&nbsp;&nbsp;<span class="green">内存限制: </span>128 MB<br>
<span class="green">提交: </span>58&nbsp;&nbsp;<span class="green">解决: </span>19<br>
[<a target="_blank" href="http://acm.neu.edu.cn/hustoj/submitpage.php?id=1212">提交</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/problemstatus.php?id=1212">状态</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/bbs.php?pid=1212">讨论版</a>]</center>
<h2>题目描述</h2>
<div class="content">&nbsp; &nbsp; &nbsp; &nbsp; 2001年9月11日，一场突发的灾难将纽约世界贸易中心大厦夷为平地，Mr.&nbsp; F曾亲&#30524;目睹了这次灾难。为了纪念“9?11”事件，Mr.&nbsp; F决定自己用水晶来搭建一座双塔。 &nbsp; &nbsp; &nbsp; &nbsp; Mr.&nbsp; F有N块水晶，每块水晶有一个高度，他想用这N块水晶搭建两座有同样高度的塔，使他们成为一座双塔，Mr.&nbsp; F可以从这N块水晶中任取M（1≤M≤N）块来搭建。但是他不知道能否使两座塔有同样的高度，也不知道如果能搭建成一座双塔，这座双塔的最大高度是多少。所以他来请你帮忙。
 &nbsp; &nbsp; &nbsp; &nbsp; 给定水晶的数量N（1≤N≤100）和每块水晶的高度Hi（N块水晶高度的总和不超过2000），你的任务是判断Mr.&nbsp; F能否用这些水晶搭建成一座双塔（两座塔有同样的高度），如果能，则输出所能搭建的双塔的最大高度，否则输出“Impossible”。
</div>
<h2>输入</h2>
<div class="content">&nbsp; &nbsp; &nbsp; &nbsp; 输入的第一行为一个数N，表示水晶的数量。第二行为N个数，第i个数表示第i个水晶的高度。 </div>
<h2>输出</h2>
<div class="content">&nbsp; &nbsp; &nbsp; &nbsp; 输出仅包含一行，如果能搭成一座双塔，则输出双塔的最大高度，否则输出一个字符串“Impossible”。 </div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">5
1 3 4 5 2
</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">7
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata"></span><pre name="code" class="cpp">#include&lt;stdio.h&gt;
int a[101];
int f[101][2002];
int height = 0;
int n;
void go(){
	int i, j;
	for (i = 0; i &lt; height; i++)f[0][i] = -1;
	f[0][0] = 0;
	for (i = 1; i &lt;= n; i++)
	for (j = 0; j &lt; height; j++){
		f[i][j] = f[i - 1][j];
		if (j - a[i] &gt;= 0){
			if (f[i - 1][j - a[i]]&gt;f[i][j])
				f[i][j] = f[i - 1][j - a[i]];
		}
		if (f[i - 1][j + a[i]] != -1)
		{
			if (f[i - 1][j + a[i]] + a[i] &gt; f[i][j])
				f[i][j] = f[i - 1][j + a[i]] + a[i];
		}
		if (a[i] - j &gt; 0){
			if (f[i - 1][a[i] - j] != -1){
				if (f[i - 1][a[i] - j] + a[i] - j&gt;f[i][j])
				f[i][j] = f[i - 1][a[i] - j] + a[i] - j;
			}
		}
	}
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	scanf(&quot;%d&quot;, &amp;n);
	int i;
	for (i = 1; i &lt;= n; i++)
	{
		scanf(&quot;%d&quot;, &amp;a[i]);
		height += a[i];
	}
	height++;
	go();
	if (f[n][0]!= 0)printf(&quot;%d\n&quot;, f[n][0]);
	else printf(&quot;Impossible\n&quot;);
	return 0;
}</pre><br><br></pre>
        