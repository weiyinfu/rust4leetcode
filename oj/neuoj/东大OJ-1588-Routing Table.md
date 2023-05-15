------------------
title = "东大OJ-1588: Routing Table"
publishTime = "2015-06-10 10:07:00"
id = "5013881"
tags = [ "算法", "东大OJ",]

--------------

<h2 style="margin:0px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; color:blue; font-size:24px; line-height:36px">
<br>
题目描述</h2>
<div class="content" style="font-family:'Times New Roman'; font-size:20px; line-height:24px; height:auto; margin:0px; padding:0px 20px; color:rgb(51,51,51); background:none 0px 0px repeat scroll rgb(228,240,248)">
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
In the computer network, a Router is a device which finds an optimal way to transmit the datagrams passing through it to it's destination efficiently. To accomplish this task, the Router maintains a Routing Table.<br>
The Routing Table stores a variety of relevant data about transmission path. In other words, the information contained in the table determines the forwarding strategy of a datagram.<br>
Normally, the information in the Routing Table for each routing table entry is:<br>
First the destination IP Address, followed by the number of bits of the sub-net mask, and finally the forwarded network port number of the destination network.<br>
For each datagram passing through it, the Router compares the datagram’s destination IP Address with the information of routing table entry, if the network number of the destination IP Address is equals to the network number stored in the routing table entry,
 then the datagram is forwarded to the corresponding port.<br>
Now, give you the Routing Table stored in the Router. Then for each datagram travel through this Router, give you it's destination IP Address, please return which network port will the datagram be forwarded to.ou</p>
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
<span style="color:rgb(255,0,0)">You will get some help:</span><a target="_blank" href="http://baike.baidu.com/link?url=r4hQpTlwEdCYIJosEHhZBX3rxuwX9l7dj7ijlsuY-7xUfJIrGuObGl0uOoBLUW5oNqXMah2QZ6OEe7elHCkoP_" style="color:rgb(26,92,200); text-decoration:none"><span style="color:rgb(255,0,0)">baike.baidu.com/link</span></a></p>
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
</p>
</div>
<h2 style="margin:0px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; color:blue; font-size:24px; line-height:36px">
输入</h2>
<div class="content" style="font-family:'Times New Roman'; font-size:20px; line-height:24px; height:auto; margin:0px; padding:0px 20px; color:rgb(51,51,51); background:none 0px 0px repeat scroll rgb(228,240,248)">
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
The first line of input contains an integer T, indicating the number of test cases (T ≤ 20).<br>
The first line of each test case contains two integers N and M, represent the number of entries in the Routing Table and the number of datagram passing through the Router, N and M are all less than 100000. Nest N lines each line represent a routing table entry,
 the format of input is IP Address/bits of sub-net mask and forwarded port number. And next M lines each line contain a destination IP Address. Please refer to the sample input for more detail.</p>
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
</p>
</div>
<h2 style="margin:0px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; color:blue; font-size:24px; line-height:36px">
输出</h2>
<div class="content" style="font-family:'Times New Roman'; font-size:20px; line-height:24px; height:auto; margin:0px; padding:0px 20px; color:rgb(51,51,51); background:none 0px 0px repeat scroll rgb(228,240,248)">
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
For each destination IP Address, please output the port number that the Router should forward. If there are many entry both match to this destination IP Address, please output the one that has the longest bits of sub-net mask. If there are also many entry match,
 please output the one that has the smallest port number. If there are none entry match, please output the default port 65535.</p>
<p style="margin-top:0px; margin-bottom:9px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; font-size:13px; line-height:18px">
</p>
</div>
<h2 style="margin:0px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; color:blue; font-size:24px; line-height:36px">
样例输入</h2>
<pre class="content" style="padding:8.5px; font-family:Menlo,Monaco,'Courier New',monospace; font-size:12.025px; color:rgb(51,51,51); margin-top:0px; margin-bottom:9px; line-height:18px; white-space:pre-wrap; word-break:break-all; word-wrap:break-word; background-color:rgb(245,245,245)"><span class="sampledata" style="font-family:monospace; font-size:18px; white-space:pre; background:none 0px 0px repeat scroll rgb(141,184,255)">1
4 4
192.168.0.0/16 1234
192.168.1.0/24 1235
192.168.1.0/23 1233
192.168.0.0/23 1236
192.168.2.0
192.168.0.0
192.168.1.0
255.255.255.255</span></pre>
<h2 style="margin:0px; font-family:'Helvetica Neue',Helvetica,Arial,sans-serif; color:blue; font-size:24px; line-height:36px">
样例输出</h2>
<pre class="content" style="padding:8.5px; font-family:Menlo,Monaco,'Courier New',monospace; font-size:12.025px; color:rgb(51,51,51); margin-top:0px; margin-bottom:9px; line-height:18px; white-space:pre-wrap; word-break:break-all; word-wrap:break-word; background-color:rgb(245,245,245)"><span class="sampledata" style="font-family:monospace; font-size:18px; white-space:pre; background:none 0px 0px repeat scroll rgb(141,184,255)">1234
1233
1235
</span><p><span class="sampledata" style="font-family:monospace; font-size:18px; white-space:pre; background:none 0px 0px repeat scroll rgb(141,184,255)">65535</span></p><p><span class="sampledata" style="font-family:monospace; font-size:18px; white-space:pre; background:none 0px 0px repeat scroll rgb(141,184,255)">
</span></p></pre>
<h4>第一次在ACM的题中手打哈希表,好高兴,一次成功了.<br>
哈希表实现要领<br>
<h5>1.哈希表中每个元素都是指针,指向某元素.<br>
2.很像链式前向星的一种结构<br>
</h5>
</h4>
<pre name="code" class="cpp">#include&lt;iostream&gt;
#include&lt;string.h&gt;
#include&lt;math.h&gt;
#include&lt;algorithm&gt;
using namespace std;
const int maxn = 1e5 + 7;
const int len = maxn * 10;
const int P = 1e9 + 7;
struct Node{
	int ip, bits,to;
	int next;
}mem[maxn];
int mi;
struct Hash{
	int a[len];
	void init(){ memset(a, 0, sizeof(a)); }
	void insert(int ip, int bits,int to){
		int pos = abs((ip*17)%P+13*bits)%len;  
		for (int i = a[pos]; i; i = mem[i].next){
			if (mem[i].ip == ip&amp;&amp;mem[i].bits == bits){
				mem[i].to = min(mem[i].to, to);
				return;
			}
		} 
		mem[++mi].ip = ip, mem[mi].bits = bits, mem[mi].to = to, mem[mi].next = a[pos], a[pos] = mi;
	}
	int get(int ip, int bits){
		int pos = abs((ip * 17) % P + 13 * bits) % len;
		for (int i = a[pos]; i; i = mem[i].next){
			if (mem[i].ip == ip&amp;&amp;mem[i].bits == bits)return mem[i].to;
		}
		return -1;
	}
}dic;
int mask[33];
void init(){ 
	mask[32] = ~0;
	for (int i = 31; i &gt;= 0; i--)mask[i] = mask[i + 1] &lt;&lt; 1;
}
int read(){
	int a, b, c, d;
	scanf(&quot;%d.%d.%d.%d&quot;, &amp;a, &amp;b, &amp;c, &amp;d);
	int ans = (a &lt;&lt; 24) | (b &lt;&lt; 16) | (c &lt;&lt; 8) | d; 
	return ans;
}
int main(){
	freopen(&quot;in.txt&quot;, &quot;r&quot;, stdin);
	init();
	int T; scanf(&quot;%d&quot;, &amp;T);
	while (T--){
		int N, M; scanf(&quot;%d%d&quot;, &amp;N, &amp;M); 
		dic.init();
		mi = 0;
		while (N--){
			int bits,to;
			int ip = read(); scanf(&quot;/%d%d&quot;, &amp;bits,&amp;to);
			ip &amp;= mask[bits];   
			dic.insert(ip, bits, to);
		}
		while (M--){
			int ip = read();
			for (int i = 32; i &gt;= 0; i--){
				ip&amp;=mask[i];
				int ans = dic.get(ip, i);
				if (ans ^-1){
					printf(&quot;%d\n&quot;, ans); goto over;
				}
			}
			printf(&quot;65535\n&quot;);
		over:;
		}
	}
	return 0;
}</pre>
        