------------------
title = "hihocoder216周：贪心或二分"
publishTime = "2018-08-20 04:55:00"
id = "9504983"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho216/problems)

有N条线段，要切K刀，使得最长的线段尽量短。在最佳切割的条件下，切完之后最长的那根绳子是多长。

# 方法一：贪心
每次切的那一刀必然是最长的那条线段，用优先队列，每次往最长的那条线段上切一刀

# 方法二：二分
假设切完之后最长的绳子长度是x，那么可以求出切多少刀来。如果刀数大于K，说明最长的绳子长度小于x。依次法可以二分答案。

```java
import java.util.Arrays;
import java.util.Comparator;
import java.util.PriorityQueue;
import java.util.Scanner;


public class Main {
class Node {
    double dis;
    int cnt;
    double per;

    Node(double dis, int cnt) {
        this.dis = dis;
        this.cnt = cnt;
        this.per = dis / cnt;
    }

    void update(int cnt) {
        this.cnt = cnt;
        this.per = dis / cnt;
    }
}

Main() {
    Scanner cin = new Scanner(System.in);
    int n = cin.nextInt(), m = cin.nextInt(), k = cin.nextInt();
    int[] a = new int[n];
    for (int i = 0; i < n; i++) a[i] = cin.nextInt();
    Arrays.sort(a);
    Node nodes[] = new Node[n - 1];
    for (int i = 0; i < nodes.length; i++) {
        nodes[i] = new Node(a[i + 1] - a[i], 1);
    }
    PriorityQueue<Node> q = new PriorityQueue<>(Comparator.comparing(x -> -x.per));
    q.addAll(Arrays.asList(nodes));
    int left = k;
    while (!q.isEmpty() && left > 0) {
        Node now = q.poll();
        now.update(now.cnt + 1);
        left--;
        q.add(now);
    }
    double ans = 0;
    for (Node i : nodes) {
        ans = Math.max(ans, i.per);
    }
    System.out.printf("%.1f", ans);
}

public static void main(String[] args) {
    new Main();
}
}
```
        