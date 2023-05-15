------------------
title = "东大oj-1511: Caoshen like math"
publishTime = "2015-06-10 14:23:00"
id = "5013880"
tags = [ "算法", "东大OJ",]

--------------

<div style="color:rgb(51,51,51); background:none 0px 0px repeat scroll rgb(228,240,248)">
<p><br>
Worfzyq&nbsp;likes&nbsp;Permutation&nbsp;problems.Caoshen&nbsp;and&nbsp;Mengjuju&nbsp;are&nbsp;expert&nbsp;at&nbsp;these&nbsp;problems&nbsp;.&nbsp;They&nbsp;have&nbsp;n&nbsp;cards,and&nbsp;all&nbsp;of&nbsp;the&nbsp;numbers&nbsp;vi&nbsp;on&nbsp;these&nbsp;cards&nbsp;are&nbsp;different&nbsp;.&nbsp;Because&nbsp;Caoshen&nbsp;doesn't</p>
<p>like&nbsp;disordered&nbsp;permutations,he&nbsp;wants&nbsp;to&nbsp;change&nbsp;the&nbsp;permutation&nbsp;into&nbsp;non-descending</p>
<p>&nbsp;permutation.He&nbsp;defines&nbsp;the&nbsp;operations:every&nbsp;time&nbsp;you&nbsp;can&nbsp;choose&nbsp;two&nbsp;digits&nbsp;casually,and</p>
<p>exchange&nbsp;the&nbsp;positions&nbsp;of&nbsp;them.Caoshen&nbsp;is&nbsp;lazy,he&nbsp;wants&nbsp;to&nbsp;know&nbsp;at&nbsp;least&nbsp;how&nbsp;many&nbsp;operations&nbsp;he</p>
<p>needs&nbsp;to&nbsp;change&nbsp;the&nbsp;permutation&nbsp;into&nbsp;non-descending&nbsp;one?</p>
</div>
<h2 style="color:blue">输入</h2>
<div style="color:rgb(51,51,51); background:none 0px 0px repeat scroll rgb(228,240,248)">
<p><span style="color:rgb(0,0,0)">There&nbsp;are&nbsp;multiple&nbsp;test&nbsp;cases</span><span style="color:rgb(0,0,0)">.</span><span style="color:rgb(0,0,0)">&nbsp;Each&nbsp;case&nbsp;contains&nbsp;a&nbsp;positive&nbsp;integer&nbsp;</span><span style="color:rgb(0,0,0)">n</span>,incicate&nbsp;the&nbsp;number&nbsp;of&nbsp;cards(n&lt;=1000000)&nbsp;.&nbsp;F<span style="color:rgb(0,0,0)">ollowed&nbsp;by&nbsp;</span><span style="color:rgb(0,0,0)">n</span><span style="color:rgb(0,0,0)">&nbsp;positive&nbsp;numbers</span>&nbsp;integers&nbsp;v1,v2,...,vn&nbsp;(1<span style="font-family:宋体">≤</span><span style="font-family:Times New Roman">vi</span><span style="font-family:宋体">≤</span><span style="font-family:Times New Roman">n)&nbsp;</span></p>
<p>---the&nbsp;value&nbsp;of&nbsp;each&nbsp;card.</p>
</div>
<h2 style="color:blue">输出</h2>
<div style="color:rgb(51,51,51); background:none 0px 0px repeat scroll rgb(228,240,248)">
<p>Print&nbsp;the&nbsp;minmum&nbsp;operations&nbsp;in&nbsp;a&nbsp;signal&nbsp;line&nbsp;for&nbsp;each&nbsp;test&nbsp;case.</p>
</div>
<h2 style="color:blue">样例输入</h2>
<pre style="color:rgb(51,51,51)"><span style="background:none 0px 0px repeat scroll rgb(141,184,255)">51 3 2 5 4</span></pre>
<h2 style="color:blue">样例输出</h2>
<pre style="color:rgb(51,51,51)"><span style="background:none 0px 0px repeat scroll rgb(141,184,255)">2</span></pre>
<pre name="code" class="cpp">#include&lt;iostream&gt;
#include&lt;stdlib.h&gt;
#include&lt;string.h&gt;
#include&lt;stdio.h&gt;
using namespace std;
const int maxn = 1e6 + 7;
struct Node{
	int v, i;
}a[maxn];
int size;
int cmp(const void *m, const void *n){
	return ((Node*)m)-&gt;v - ((Node*)n)-&gt;v;
}
bool vis[maxn];
int dfs(int x){
	vis[x] = true; 
	if (vis[a[x].i])return 0;
	else return 1 + dfs(a[x].i);
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	while (~scanf(&quot;%d&quot;, &amp;size)){
		for (int i = 0; i &lt; size; i++)scanf(&quot;%d&quot;,&amp; a[i].v), a[i].i = i;
		qsort(a, size, sizeof(Node), cmp);
		memset(vis, 0, sizeof(vis));
		long long int ans = 0;
		for (int i = 0; i &lt; size; i++){
			if (!vis[i])ans += dfs(i);
		}
		printf(&quot;%lld\n&quot;, ans);
	}
	return 0;
}</pre>
        