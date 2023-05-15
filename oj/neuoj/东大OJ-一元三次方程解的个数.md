------------------
title = "东大OJ-一元三次方程解的个数"
publishTime = "2014-07-31 12:14:00"
id = "5013901"
tags = [ "算法", "东大OJ",]

--------------

<center>
<h2>1043: Fixed Point</h2>
<span class="green">时间限制: </span>5 Sec&nbsp;&nbsp;<span class="green">内存限制: </span>128 MB<br>
<span class="green">提交: </span>26&nbsp;&nbsp;<span class="green">解决: </span>5<br>
[<a target="_blank" href="http://acm.neu.edu.cn/hustoj/submitpage.php?id=1043">提交</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/problemstatus.php?id=1043">状态</a>][<a target="_blank" href="http://acm.neu.edu.cn/hustoj/bbs.php?pid=1043">讨论版</a>]</center>
<h2>题目描述</h2>
<div class="content">
<p style="margin-bottom:0.19in"><span style="font-family:微软雅黑"><span style="font-size:14px">In
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">mathematics</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">, a
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">fixed point</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> (sometimes shortened to
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">fixpoint</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">, also known as an
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">invariant point</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">) of a</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">
 function </span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">is a point that is mapped to itself by the function. That is to say,
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> is a fixed point of the function
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">if and only if
</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">(</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">)
 = </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">. For example, if
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> is defined on the</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">
 real numbers </span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">by</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US"> f(x)=x*x-3*x&#43;4.,</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">then
 2 is a fixed point of </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">, because
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">(2) = 2.</span></span></p>
<p style="margin-bottom:0.19in; margin-top:0.19in"><span style="font-family:微软雅黑"><span style="font-size:14px">Not all functions have fixed points: for example, if
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> is a function defined on the real numbers as
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">(</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">)
 = </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> &#43; 1, then it has no fixed points, since
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> is never equal to
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> &#43; 1 for any real number. In graphical terms, a fixed point means the point (</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">,
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">(</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">))
 is on the line </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>y</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> =
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">, or in other words the</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">
 graph </span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">of
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US"></span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">has a point in common with that line. The example
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>f</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">(</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">)
 = </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><em>x</em></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> &#43; 1 is a case where the graph and the line are a pair of
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">parallel
</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">lines.</span></span></p>
<p style="margin-bottom:0.19in; margin-top:0.19in"><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">------
</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">http://en.wikipedia.org/wiki/Fixed_point_%28mathematics%29</span></span></p>
<p lang="en-US" style="margin-bottom:0.19in; margin-top:0.19in"><span style="font-family:微软雅黑"><span style="font-size:14px">Our problem is,for a defined on real number function:
</span></span></p>
<p lang="en-US" style="margin-bottom:0in; margin-top:0.19in"><span style="font-family:微软雅黑"><span style="font-size:14px">f(x)=a*x*x*x&#43;b*x*x&#43;c*x&#43;d,how many different fixed points does it have? &nbsp; &nbsp;</span></span></p>
<p></p>
</div>
<h2>输入</h2>
<div class="content">
<p style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">There are multiple test cases.</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">For each test cases, there are four
 integers a, b, </span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">c</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> and
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">d</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px"> in a single line.</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><br>
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px">You can assume that
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">-2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">&lt;=a&lt;=2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px">,
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">-2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">&lt;=b&lt;=2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px">,
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">-2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">&lt;=c&lt;=2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px">,
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">-2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">&lt;=d&lt;=2</span></span></span><sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">13</span></span></span></sup><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US">,and
 the number of the result is countable.</span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">
</span></span></p>
<p></p>
</div>
<h2>输出</h2>
<div class="content">
<p style="margin-bottom:0.17in"><span style="font-family:微软雅黑"><span style="font-size:14px">For each test case, output the answer
</span></span><span style="font-family:微软雅黑"><span style="font-size:14px"><span lang="en-US"></span></span></span><span style="font-family:微软雅黑"><span style="font-size:14px">in a single line.</span></span></p>
<p></p>
</div>
<h2>样例输入</h2>
<pre class="content"><span class="sampledata">3 111 793 -3456
5 -135 811 0
-1 9 10 -81</span></pre>
<h2>样例输出</h2>
<pre class="content"><span class="sampledata">3
3
3</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata">
</span></pre>
<pre class="content"><span class="sampledata"></span><pre name="code" class="cpp">#include&lt;iostream&gt;
#include&lt;math.h&gt;
using namespace std;
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	double a3, b3, c3, d3;
	double a2, b2, c2;
	while (cin &gt;&gt; a3 &gt;&gt; b3 &gt;&gt; c3 &gt;&gt; d3){
		c3--;
		a2 = 3 * a3; b2 = 2 * b3; c2 = c3;
		if (a3 == 0){
			if (b3 == 0)//一次方程
			{
				if (c3 == 0){//只剩常数
					cout &lt;&lt; 0 &lt;&lt; endl;
				}
				else cout &lt;&lt; 1 &lt;&lt; endl;
			}
			else{//二次方程
				double delta = c3*c3 - 4 * b3*d3;
				if (delta == 0)cout &lt;&lt; 1 &lt;&lt; endl;
				else if (delta&gt;0)cout &lt;&lt; 2 &lt;&lt; endl;
				else cout &lt;&lt; 0 &lt;&lt; endl;
			}
		}
		else{//三次方程
			double delta = b2*b2 - 4 * a2*c2;
			if (delta &lt;= 0)//单调递增
				cout &lt;&lt; 1 &lt;&lt; endl;
			else{
				double x1 = (-b2 - sqrt(delta)) / (2 * a2);
				double x2 = (-b2 + sqrt(delta)) / (2 * a2);
				double y1 = a3*x1*x1*x1 + b3*x1*x1 + c3*x1 + d3;
				double y2 = a3*x2*x2*x2 + b3*x2*x2 + c3*x2 + d3;
				double see = y1*y2;
				if (see == 0)cout &lt;&lt; 2 &lt;&lt; endl;
				else if (see &gt; 0)cout &lt;&lt; 1 &lt;&lt; endl;
				else cout &lt;&lt; 3 &lt;&lt; endl;
			}
		}
	}
	return 0;
}</pre><br><br></pre>
        