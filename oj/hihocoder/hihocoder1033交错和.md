------------------
title = "hihocoder1033交错和"
publishTime = "2015-10-22 11:48:00"
id = "4902595"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/problemset/problem/1033)

坑:

1.`ll x,y;z=x*y;`可能会溢出,于是我写成`x%mod*y%mod`

仍旧错误,要写成`x%mod*(y%mod)`.

2.`f(9019)=1.`
要注意如果为0,下一位的符号根据0的个数而变化


```cpp
#include<iostream>
#include<stdio.h>
#include<algorithm>
#include<queue>
#include<math.h>
#include<string.h>
#include<string>
#include<stdlib.h>
using namespace std;
typedef long long ll;
typedef unsigned long long ull;
#define re(i,n) for(int i=0;i<n;i++)      
const int mod = 1e9 + 7;
/*
Node cnt表示个数,sum表示和
*/
struct Node{
    ll cnt, sum;
    Node() :cnt(0), sum(0){}
    Node(ll c, ll s) :cnt(c), sum(s){}
}dp[19][2][2][600];//(bits,+-,+-can change,sum)
/*
19 最多19位
2  +还是-
2  是否是第一个数字
600 f(n)的结果,因为19位*9=171,输入的k是-100到100,所以范围大概是-300到+300,所以用600
*/
ll ten[19];
//ten[i]表示10^i
void init(){
    ten[0] = 1;
    for (int i = 1; i < 19; i++){
        ten[i] = 10 * ten[i - 1];
    } 
}
/*d是一个大管家,管理者dp这个数组,如果计算过,那就不再计算了
对参数做一些处理,像适配器一样
n表示[0,10^n-1]范围内,第一个符号为flag,符号是否会改变change,f(n)=k
*/
Node d(int n, int flag, int change, int k){
    int ff = (flag == 1 ? 1 : 0);
    int kk = k + 300;
    Node f(ll,int, int, int);
    if (dp[n][ff][change][kk].cnt == -1){
        dp[n][ff][change][kk] = f(ten[n] - 1, flag, change, k);
    }
    return dp[n][ff][change][kk];
}
/*
主要逻辑都在这个函数里面,n表示[0,n]范围内的值
*/
Node f(ll n, int flag, int change, int k){ 
    if (n < 0)return Node(0, 0);
    if (n < 10){
        if (flag*k >= 0 && flag*k <= n)return Node(1, flag*k);
        else return Node(0, 0);
    }
    ll mi = 0, h = 0; 
    for (ll tmp = n; tmp; tmp /= 10)h = tmp % 10, mi++;
    mi--;
    ll va = h*ten[mi];//处理清楚最高位
    Node t = d(mi, change ? 1 : -flag, change, k); //第一位为0时,有多少种情况
    Node ans = t;
    for (int i = 1; i < h; i++){
        t = d(mi, -flag, 0, k - flag*i); 
        ans.cnt += t.cnt;
        ans.sum = (ans.sum + ten[mi]%mod*i*( t.cnt%mod) + t.sum) % mod;
    }
    //第一位为h时的情况
    int ff = ((mi&1)?flag:-flag);
    for (ll tmp=n-va; tmp; tmp/=10){
        ff *= -1;  
    }
    t = f(n - va, ff, 0, k - flag*h); 
    ans.cnt += t.cnt;
    ans.sum = (ans.sum + va%mod*(t.cnt%mod) + t.sum) % mod;
    return ans;
} 
int main(){
    //freopen("in.txt", "r", stdin);
    init();
    memset(dp, -1, sizeof(dp));
    ll l, r, k;
    cin >> l >> r >> k;
    Node m = f(l - 1, 1,1, k), n = f( r, 1,1, k);
    ll ans = (n.sum - m.sum) % mod;
    if (ans < 0)ans += mod; 
    cout << ans << endl; 
    return 0;
}
```


别人的代码还精简,算法更好.

```cpp
struct node  
{  
    ll cnt,sum; //分别表示该状态的出现的次数，以及数字和  
    node(ll _cnt,ll _sum):cnt(_cnt),sum(_sum){}  
    node(){}  
}dp[20][20][300]; //dp[i][j][k],表示当前在的i位，第一位有效位为j，交错和为k-100的状态  
ll num[20];  
ll ten[20];  
ll l,r;  
int k;  
node dfs(int cur,int first,int sum,bool limit)  
{  
    if(cur <= 0)  
        return node(sum==k,0);  
    if(!limit && dp[cur][first][sum+100].cnt != -1) return dp[cur][first][sum+100];  
    int up = limit ? num[cur] : 9;  
    node ret(0,0),tv;  
    rep(i,up+1)  
    {  
        int g;  
        if(!first){ g = (i == 0 ? 0 : cur); }  
        else g = first;  
        if(g)  
            tv = dfs(cur-1,g,sum+((g-cur)%2==0?1:-1)*i,limit && i == up);  
        else  
            tv = dfs(cur-1,0,0,limit && i==up);  
        ll t = i*ten[cur-1]%mod;  
        ret.cnt = (ret.cnt + tv.cnt) % mod; //次数相加  
        ret.sum = (ret.sum + tv.sum + t*tv.cnt)%mod; //和相加  
    }  
    if(!limit) dp[cur][first][sum+100] = ret;  
    return ret;  
}  
ll solve(ll n)  
{  
    if(n <= 0) return 0;  
    int len = 0;  
    while(n){  
        num[++len] = n % 10;  
        n /= 10;  
    }  
    return dfs(len,0,0,1).sum;  
}  
void init()  
{  
    memset(dp,-1,sizeof(dp));  
    ten[0] = 1;  
    for(int i=1;i<20;i++) ten[i] = (ten[i-1] * 10) % mod;  
}  
int main()  
{  
#ifndef ONLINE_JUDGE  
    freopen("in.txt","r",stdin);  
    // freopen("out.txt","w",stdout);  
#endif    
    init();  
    while(~scanf("%lld%lld%d",&l,&r,&k))  
    {  
        cout<<(solve(r)-solve(l-1)+mod)%mod<<'\n';  
    }  
    return 0;  
}  
```

        