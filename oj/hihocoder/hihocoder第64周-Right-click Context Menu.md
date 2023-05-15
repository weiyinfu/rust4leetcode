------------------
title = "hiho一下 第六十四周\tRight-click Context Menu"
publishTime = "2015-09-22 15:20:00"
id = "4830884"
tags = [ "算法", "hihocoder",]

--------------

[题目链接:hihocoder 第64周](http://hihocoder.com/contest/hiho64/problem/1)

# 题意概述
上下文菜单是panel(面板)包括很多section(分区),一个分区里面至少包含一个菜单项.每一个菜单项都对应有一个子panel,这个panel可能为空.也可能又是一个新的包含很多分区的panel.如何安排同一个section内的菜单项顺序?如何安排同一个panel内的各个分区?才能使的展开之后最短,占的空间最少?

# 方法
子级菜单长度-我的菜单长度=h,对h进行排序.h大的在上面.

```cpp
#include<iostream>
#include<vector>
#include<algorithm>
#include<stdio.h>
#include<string.h>
#include<string>
using namespace std;
#define ll long long 
#define re(i,n) for(int i=0;i<n;i++)
const int maxn = 1007;
int n;
/*
Sec类是一个分区,分区包含很多菜单项;
每个菜单项是一个Panel.
a.size()表示这个分区所包含的菜单个数
h表示子菜单比本体长多少
*/
struct Sec{
    vector<int>a;
    int h; 
};
/*
这个vector表示全部的panel.一个菜单项必然对应一个panel.
h表示子菜单比本体长多少
cnt表示此panel所包含的菜单项数
*/
vector<Sec>a[maxn];
int h[maxn],cnt[maxn];
bool cmp(const Sec&m, const Sec&n){
    return m.h > n.h;
}
bool cm(const int&m, const int &n){
    return h[m] > h[n];
}
void go(int);
void go(Sec&sec){
    re(i, sec.a.size())go(sec.a[i]);
    sort(sec.a.begin(), sec.a.end(), cm);
    sec.h = 0;
    int sz = sec.a.size();
    re(i, sz)sec.h = max(i + h[sec.a[i]] - sz, sec.h); 
}
void go(int r){
    re(i, a[r].size())go(a[r][i]);
    sort(a[r].begin(), a[r].end(), cmp);
    int now = 0;
    h[r] = cnt[r];
    int sz = a[r].size();
    re(i, sz){
        int self = a[r][i].a.size();//第i个sec的包含的菜单数
        now += self;
        h[r] = max(now +a[r][i].h, h[r]);
    }
} 
int main(){
    freopen("in.txt", "r", stdin);
    cin >> n;
    re(i, n + 1){ 
        scanf("%d", &cnt[i]);
        Sec sec;
        re(j, cnt[i]){
            int x; scanf("%d", &x);
            if (x == 0){
                j--;
                a[i].push_back(sec);
                sec.a.clear();
                continue;
            }
            sec.a.push_back(x);
        }
        a[i].push_back(sec);
    }
    go(0);  
    printf("%d\n", h[0]);
    return 0;
}
```