------------------
title = "hiho一下 第六十六周"
publishTime = "2015-10-04 13:49:00"
id = "4855064"
tags = [ "算法", "hihocoder",]

--------------

<h3>题目链接:<a href="http://hihocoder.com/contest/hiho66/problem/1" target="_blank">这是一道水爆了的广搜题</a></h3>
<div class="cnblogs_code">
<pre>#include&lt;iostream&gt;<span style="color: #000000;">
#include</span>&lt;stdio.h&gt;<span style="color: #000000;">
#include</span>&lt;algorithm&gt;<span style="color: #000000;">
#include</span>&lt;<span style="color: #0000ff;">string</span>.h&gt;
<span style="color: #0000ff;">using</span> <span style="color: #0000ff;">namespace</span><span style="color: #000000;"> std;
</span><span style="color: #0000ff;">const</span> <span style="color: #0000ff;">int</span> maxn = <span style="color: #800080;">2</span> * 1e5 + <span style="color: #800080;">7</span><span style="color: #000000;">;
typedef </span><span style="color: #0000ff;">long</span> <span style="color: #0000ff;">long</span><span style="color: #000000;"> ll;
</span><span style="color: #0000ff;">#define</span> re(i,n) for(int i=0;i&lt;n;i++) 
<span style="color: #0000ff;">char</span> a[<span style="color: #800080;">107</span>][<span style="color: #800080;">107</span><span style="color: #000000;">];
</span><span style="color: #0000ff;">int</span> b[<span style="color: #800080;">107</span>][<span style="color: #800080;">107</span><span style="color: #000000;">];
</span><span style="color: #0000ff;">int</span><span style="color: #000000;"> n, m;
</span><span style="color: #0000ff;">int</span><span style="color: #000000;"> sx, sy;
</span><span style="color: #0000ff;">struct</span><span style="color: #000000;"> Point{
    </span><span style="color: #0000ff;">int</span><span style="color: #000000;"> x, y;
};
</span><span style="color: #0000ff;">struct</span><span style="color: #000000;"> Q{
    Point a[</span><span style="color: #800080;">10000</span><span style="color: #000000;">];
    </span><span style="color: #0000ff;">int</span><span style="color: #000000;"> h, r;
    </span><span style="color: #0000ff;">void</span> init(){ h = r = <span style="color: #800080;">0</span><span style="color: #000000;">; }
    </span><span style="color: #0000ff;">void</span> enq(<span style="color: #0000ff;">int</span> x, <span style="color: #0000ff;">int</span><span style="color: #000000;"> y){
        a[r].x </span>= x, a[r].y = y, r++<span style="color: #000000;">;
    }
    Point deq(){ 
        </span><span style="color: #0000ff;">return</span> a[h++<span style="color: #000000;">];
    }
}q;
</span><span style="color: #0000ff;">int</span> dir[<span style="color: #800080;">4</span>][<span style="color: #800080;">2</span>] = {<span style="color: #800080;">0</span>,<span style="color: #800080;">1</span>,<span style="color: #800080;">1</span>,<span style="color: #800080;">0</span>,<span style="color: #800080;">0</span>,-<span style="color: #800080;">1</span>,-<span style="color: #800080;">1</span>,<span style="color: #800080;">0</span><span style="color: #000000;">};
</span><span style="color: #0000ff;">void</span><span style="color: #000000;"> go(){
    memset(b, </span>-<span style="color: #800080;">1</span>, <span style="color: #0000ff;">sizeof</span><span style="color: #000000;">(b));
    b[sx][sy] </span>= <span style="color: #800080;">0</span><span style="color: #000000;">;
    q.init();
    q.enq(sx, sy);
    </span><span style="color: #0000ff;">while</span> (q.h !=<span style="color: #000000;"> q.r){
        Point me </span>=<span style="color: #000000;"> q.deq();
        re(i, </span><span style="color: #800080;">4</span><span style="color: #000000;">){
            </span><span style="color: #0000ff;">int</span> x = me.x + dir[i][<span style="color: #800080;">0</span>], y = me.y + dir[i][<span style="color: #800080;">1</span><span style="color: #000000;">];
            </span><span style="color: #0000ff;">if</span> (b[x][y] != -<span style="color: #800080;">1</span>)<span style="color: #0000ff;">continue</span><span style="color: #000000;">;
            </span><span style="color: #0000ff;">if</span> (a[x][y] == <span style="color: #800000;">'</span><span style="color: #800000;">.</span><span style="color: #800000;">'</span><span style="color: #000000;">){ 
                b[x][y] </span>= b[me.x][me.y] + <span style="color: #800080;">1</span><span style="color: #000000;">;
                q.enq(x, y); 
            }
            </span><span style="color: #0000ff;">else</span> <span style="color: #0000ff;">if</span> (a[x][y] == <span style="color: #800000;">'</span><span style="color: #800000;">S</span><span style="color: #800000;">'</span><span style="color: #000000;">){
                b[x][y] </span>=  b[me.x][me.y] + <span style="color: #800080;">1</span><span style="color: #000000;">;
            }
        }
    }
}
</span><span style="color: #0000ff;">int</span><span style="color: #000000;"> main(){
    freopen(</span><span style="color: #800000;">"</span><span style="color: #800000;">in.txt</span><span style="color: #800000;">"</span>, <span style="color: #800000;">"</span><span style="color: #800000;">r</span><span style="color: #800000;">"</span><span style="color: #000000;">, stdin);
    cin </span>&gt;&gt; n &gt;&gt;<span style="color: #000000;"> m;
    re(i, n)re(j, m){
        cin </span>&gt;&gt; a[i + <span style="color: #800080;">1</span>][j + <span style="color: #800080;">1</span><span style="color: #000000;">]; 
        </span><span style="color: #0000ff;">if</span> (a[i + <span style="color: #800080;">1</span>][j + <span style="color: #800080;">1</span>] == <span style="color: #800000;">'</span><span style="color: #800000;">P</span><span style="color: #800000;">'</span>)a[i + <span style="color: #800080;">1</span>][j + <span style="color: #800080;">1</span>] = <span style="color: #800000;">'</span><span style="color: #800000;">#</span><span style="color: #800000;">'</span><span style="color: #000000;">;
        </span><span style="color: #0000ff;">else</span> <span style="color: #0000ff;">if</span> (a[i + <span style="color: #800080;">1</span>][j + <span style="color: #800080;">1</span>] == <span style="color: #800000;">'</span><span style="color: #800000;">H</span><span style="color: #800000;">'</span><span style="color: #000000;">){
            sx </span>= i + <span style="color: #800080;">1</span>, sy = j + <span style="color: #800080;">1</span><span style="color: #000000;">;
            a[sx][sy] </span>= <span style="color: #800000;">'</span><span style="color: #800000;">.</span><span style="color: #800000;">'</span><span style="color: #000000;">;
        }
    }
    re(i, n </span>+ <span style="color: #800080;">1</span>)a[i][m + <span style="color: #800080;">1</span>] = a[i][<span style="color: #800080;">0</span>] = <span style="color: #800000;">'</span><span style="color: #800000;">#</span><span style="color: #800000;">'</span><span style="color: #000000;">;
    re(i, m </span>+ <span style="color: #800080;">1</span>)a[<span style="color: #800080;">0</span>][i] = a[n + <span style="color: #800080;">1</span>][i] = <span style="color: #800000;">'</span><span style="color: #800000;">#</span><span style="color: #800000;">'</span><span style="color: #000000;">;
    go();</span><span style="color: #008000;">/*</span><span style="color: #008000;">
    re(i, n + 1){
        re(j, m + 1){
            printf("%3d", b[i][j]);
        }
        puts("");
    }</span><span style="color: #008000;">*/</span>
    <span style="color: #0000ff;">int</span> ans =<span style="color: #000000;"> 1e5;
    </span><span style="color: #0000ff;">for</span> (<span style="color: #0000ff;">int</span> i = <span style="color: #800080;">1</span>; i &lt;= n; i++<span style="color: #000000;">){
        </span><span style="color: #0000ff;">for</span> (<span style="color: #0000ff;">int</span> j = <span style="color: #800080;">1</span>; j &lt;= m; j++<span style="color: #000000;">){
            </span><span style="color: #0000ff;">if</span> (a[i][j] == <span style="color: #800000;">'</span><span style="color: #800000;">S</span><span style="color: #800000;">'</span><span style="color: #000000;">){
                </span><span style="color: #0000ff;">if</span> (b[i][j] == -<span style="color: #800080;">1</span>)<span style="color: #0000ff;">continue</span><span style="color: #000000;">;
                re(k, </span><span style="color: #800080;">4</span><span style="color: #000000;">){
                    </span><span style="color: #0000ff;">int</span> x = i + dir[k][<span style="color: #800080;">0</span>], y = j + dir[k][<span style="color: #800080;">1</span><span style="color: #000000;">];
                    </span><span style="color: #0000ff;">if</span> (a[x][y] == <span style="color: #800000;">'</span><span style="color: #800000;">S</span><span style="color: #800000;">'</span><span style="color: #000000;">){
                        </span><span style="color: #0000ff;">if</span> (b[x][y] != -<span style="color: #800080;">1</span><span style="color: #000000;">){
                            </span><span style="color: #0000ff;">int</span> now = b[x][y] +<span style="color: #000000;"> b[i][j];
                            ans </span>=<span style="color: #000000;"> min(now, ans);
                        }
                    }
                }
            }
        }
    }
    </span><span style="color: #0000ff;">if</span> (ans ==<span style="color: #000000;"> 1e5){
        puts(</span><span style="color: #800000;">"</span><span style="color: #800000;">Hi and Ho will not have lunch.</span><span style="color: #800000;">"</span><span style="color: #000000;">);
    }
    </span><span style="color: #0000ff;">else</span><span style="color: #000000;">{
        printf(</span><span style="color: #800000;">"</span><span style="color: #800000;">%d\n</span><span style="color: #800000;">"</span><span style="color: #000000;">, ans);
    }
    </span><span style="color: #0000ff;">return</span> <span style="color: #800080;">0</span><span style="color: #000000;">;
}</span></pre>
</div>
<p>&nbsp;</p>
        