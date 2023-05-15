------------------
title = "hihocoder1241 Best Route in a Grid"
publishTime = "2015-10-18 18:35:00"
id = "4890794"
tags = [ "算法", "hihocoder",]

--------------

<h3>题目链接:<a href="http://hihocoder.com/problemset/problem/1241?sid=609675" target="_blank">hihocoder 1241</a></h3>
<h3>题意:</h3>
<p>n*n的格阵,每个方格内有一个数字.蚂蚁从左上角走到右下角,数字是零的方格不能走,只能向右向下走.蚂蚁走的路径上全部方格的的乘积为s,要使s低位0的个数尽量少.问,最少s的末尾包含几个0.</p>
<h3>分析:</h3>
<p>10=2*5,所以只要统计蚂蚁路径上2的个数和5的个数,二者之中的较小者即为s末尾0的个数.</p>
<p>假设2的个数为x,5的个数为y.</p>
<p>对于路径(x,y),答案是min(x,y).&nbsp;</p>
<p>"路径(p,q)比路径(x,y)好"的充要条件"min(p,q)&lt;min(x,y)".</p>
<p>最优路径(x,y)中x为最小值或者y为最小值.</p>
<p>&nbsp;</p>
<p>这个问题可以进行延伸:将路径上数字的乘积用k进制来表示,使得末尾0的个数尽量少.对k进行因子分解,当k是若干个质数的乘积时,k=p1*p2*p3*p4...,对于每一个pi进行一次动归.当上边的主键等于左边时,就应该比较剩余的值,有点不好整了,还是两个质数之积比较简单.当k=p1^m1*p2^m2......时,又该怎么做呢?</p>
<h3>代码:</h3>
<div class="cnblogs_code">
<pre><span style="color: #008080;"> 1</span> #include&lt;iostream&gt;
<span style="color: #008080;"> 2</span> #include&lt;stdio.h&gt;
<span style="color: #008080;"> 3</span> #include&lt;algorithm&gt;
<span style="color: #008080;"> 4</span> #include&lt;queue&gt;
<span style="color: #008080;"> 5</span> #include&lt;math.h&gt;
<span style="color: #008080;"> 6</span> #include&lt;string.h&gt;
<span style="color: #008080;"> 7</span> #include&lt;stdlib.h&gt;
<span style="color: #008080;"> 8</span> <span style="color: #000000;">using namespace std;
</span><span style="color: #008080;"> 9</span> <span style="color: #000000;">typedef long long ll;
</span><span style="color: #008080;">10</span> <span style="color: #000000;">typedef unsigned long long ull;
</span><span style="color: #008080;">11</span> #define re(i,n) <span style="color: #0000ff;">for</span>(int i=<span style="color: #800080;">0</span>;i&lt;n;i++<span style="color: #000000;">)   
</span><span style="color: #008080;">12</span> <span style="color: #000000;">int n;
</span><span style="color: #008080;">13</span> <span style="color: #0000ff;">const</span> int maxn = <span style="color: #800080;">1007</span><span style="color: #000000;">;
</span><span style="color: #008080;">14</span> <span style="color: #000000;">int a[maxn][maxn];
</span><span style="color: #008080;">15</span> <span style="color: #000000;">int two[maxn][maxn], five[maxn][maxn];
</span><span style="color: #008080;">16</span> int s[maxn][maxn][<span style="color: #800080;">2</span><span style="color: #000000;">];
</span><span style="color: #008080;">17</span> void go(int c[maxn][maxn], int x)<span style="color: #008000;">{</span> 
<span style="color: #008080;">18</span> <span style="color: #008000;">    for (int i = 1; i &lt;= n; i++){
</span><span style="color: #008080;">19</span> <span style="color: #008000;">        for (int j = 1; j &lt;= n; j++){
</span><span style="color: #008080;">20</span> <span style="color: #008000;">            if (a[i][j] == 0){
</span><span style="color: #008080;">21</span> <span style="color: #008000;">                c[i][j] = 1e6; continue;
</span><span style="color: #008080;">22</span>             <span style="color: #008000;">}</span>
<span style="color: #008080;">23</span>             int cnt = <span style="color: #800080;">0</span><span style="color: #000000;">;
</span><span style="color: #008080;">24</span>             <span style="color: #0000ff;">for</span> (int k = a[i][j]; k%x == <span style="color: #800080;">0</span>; k /= x)cnt++<span style="color: #000000;">;
</span><span style="color: #008080;">25</span>             c[i][j] =<span style="color: #000000;"> cnt;
</span><span style="color: #008080;">26</span> <span style="color: #000000;">        }
</span><span style="color: #008080;">27</span> <span style="color: #000000;">    }
</span><span style="color: #008080;">28</span> <span style="color: #000000;">}
</span><span style="color: #008080;">29</span> void work(int c[maxn][maxn], int cc[maxn][maxn])<span style="color: #008000;">{</span>
<span style="color: #008080;">30</span> <span style="color: #008000;">    re(i, n + 1)s[0][i][0] = s[0][i][1] = s[i][0][0] = s[i][0][1] = 1e6;
</span><span style="color: #008080;">31</span> <span style="color: #008000;">    s[0][1][0] = s[0][1][1] = s[1][0][0] = s[1][0][1] = 0;
</span><span style="color: #008080;">32</span> <span style="color: #008000;">    for (int i = 1; i &lt;= n; i++){
</span><span style="color: #008080;">33</span> <span style="color: #008000;">        for (int j = 1; j &lt;= n; j++){
</span><span style="color: #008080;">34</span> <span style="color: #008000;">            if (s[i - 1][j][0] == s[i][j - 1][0]){
</span><span style="color: #008080;">35</span> <span style="color: #008000;">                s[i][j][0] = c[i][j] + s[i][j - 1][0];
</span><span style="color: #008080;">36</span> <span style="color: #008000;">                s[i][j][1] = cc[i][j]+min(s[i][j - 1][1], s[i - 1][j][1]);
</span><span style="color: #008080;">37</span>             <span style="color: #008000;">}</span>
<span style="color: #008080;">38</span>             <span style="color: #0000ff;">else</span> <span style="color: #0000ff;">if</span> (s[i - <span style="color: #800080;">1</span>][j][<span style="color: #800080;">0</span>]&lt; s[i][j - <span style="color: #800080;">1</span>][<span style="color: #800080;">0</span>])<span style="color: #008000;">{</span>
<span style="color: #008080;">39</span> <span style="color: #008000;">                s[i][j][0] = s[i - 1][j][0] + c[i][j];
</span><span style="color: #008080;">40</span> <span style="color: #008000;">                s[i][j][1] = s[i - 1][j][1] + cc[i][j];
</span><span style="color: #008080;">41</span>             <span style="color: #008000;">}</span>
<span style="color: #008080;">42</span>             <span style="color: #0000ff;">else</span><span style="color: #008000;">{</span>
<span style="color: #008080;">43</span> <span style="color: #008000;">                s[i][j][0] = s[i][j-1][0] + c[i][j];
</span><span style="color: #008080;">44</span> <span style="color: #008000;">                s[i][j][1] = s[i][j-1][1] + cc[i][j];
</span><span style="color: #008080;">45</span>             <span style="color: #008000;">}</span>
<span style="color: #008080;">46</span> <span style="color: #000000;">        }
</span><span style="color: #008080;">47</span> <span style="color: #000000;">    }
</span><span style="color: #008080;">48</span> <span style="color: #000000;">}  
</span><span style="color: #008080;">49</span> int main()<span style="color: #008000;">{</span>
<span style="color: #008080;">50</span> <span style="color: #008000;">    freopen("in.txt", "r", stdin);
</span><span style="color: #008080;">51</span> <span style="color: #008000;">    cin &gt;&gt; n;
</span><span style="color: #008080;">52</span> <span style="color: #008000;">    re(i, n)re(j, n)scanf("%d", &amp;a[i + 1][j + 1]);
</span><span style="color: #008080;">53</span> <span style="color: #008000;">    go(two, 2), go(five, 5);  
</span><span style="color: #008080;">54</span> <span style="color: #008000;">    work(two, five); int p = min(s[n][n][0], s[n][n][1]);
</span><span style="color: #008080;">55</span> <span style="color: #008000;">    work(five, two); int q = min(s[n][n][0], s[n][n][1]);
</span><span style="color: #008080;">56</span> <span style="color: #008000;">    cout &lt;&lt; min(p, q) &lt;&lt; endl;
</span><span style="color: #008080;">57</span> <span style="color: #008000;">    return 0;
</span><span style="color: #008080;">58</span> <span style="color: #008000;">}</span></pre>
</div>
<p>&nbsp;</p>
        