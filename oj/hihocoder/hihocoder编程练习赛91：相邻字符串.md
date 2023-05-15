------------------
title = "hihocoder编程练习赛91：相邻字符串"
publishTime = "2019-01-20 06:39:00"
id = "10294854"
tags = [ "算法",]

--------------

[题目链接](http://hihocoder.com/contest/offers91/problem/3)

给定一个长度小于1e5的字符串s，s中字符全是大写英语字母。现在要寻找s中有多少组邻近的“hio”字符串，邻近的定义如下：hi距离+io距离+ho距离小于k。输入k和s，求有多少组邻近的hio。

此题关键在于字符串是一维的序列，hi距离+io距离+ho距离必然是偶数，此距离必为hio中最左端字符和最右端字符距离的二倍。

由此，对于任意最左端字符，只需要保证最右端字符和最左端距离不超过k/2即可。使用二分查找解决。

```java
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Main {
int upperBound(List<Integer> a, int x) {
    int l = 0, r = a.size();
    while (l + 1 < r) {
        int mid = (l + r) >> 1;
        if (a.get(mid) <= x) {
            l = mid;
        } else if (a.get(mid) > x) {
            r = mid;
        }
    }
    if (r < a.size() && a.get(r) <= x) r++;
    return r;
}

int lowerBound(List<Integer> a, int x) {
    int l = 0, r = a.size();
    while (l + 1 < r) {
        int mid = (l + r) >> 1;
        if (a.get(mid) < x) {
            l = mid;
        } else if (a.get(mid) >= x) {
            r = mid;
        }
    }
    if (a.get(l) >= x) l--;
    return l;
}

long query(List<Integer> a, int beg, int end) {
    int b = lowerBound(a, beg), e = upperBound(a, end);
    return e - b - 1;
}

Main() {
    Scanner cin = new Scanner(System.in);
    int k = cin.nextInt();
    char[] a = cin.next().trim().toLowerCase().toCharArray();
    List<Integer> h = new ArrayList<>(a.length), i = new ArrayList<>(a.length), o = new ArrayList<>(a.length);
    List<Integer> hio = new ArrayList<>(a.length);
    for (int j = 0; j < a.length; j++) {
        if (a[j] == 'h') {
            h.add(j);
            hio.add(j);
        } else if (a[j] == 'i') {
            i.add(j);
            hio.add(j);
        } else if (a[j] == 'o') {
            o.add(j);
            hio.add(j);
        }
    }
    k /= 2;
    long s = 0;
    for (int j : hio) {
        if (a[j] == 'h') {
            s += query(i, j, j + k) * query(o, j, j + k);
        } else if (a[j] == 'i') {
            s += query(h, j, j + k) * query(o, j, j + k);
        } else if (a[j] == 'o') {
            s += query(h, j, j + k) * query(i, j, j + k);
        }
    }
    System.out.println(s);
}

public static void main(String[] args) {
    new Main();
}
}

```
        