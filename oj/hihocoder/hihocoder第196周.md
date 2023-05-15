------------------
title = "hihocoder第196周"
publishTime = "2018-04-01 11:52:00"
id = "8687924"
tags = [ "算法", "hihocoder",]

--------------

此题解法：动态规划，倒骑毛驴。
在使用动态规划的时候，如果正着求难求，可以考虑倒着来。

这道题坑不少，自己代码能力太弱了，写代码的过程中总是容易犯细节错误。虽然大的方向是对的，但是小坑非常致命！
比如一开始下面这两句话写反了。
```java
nowHeight = (int) Math.max(nowHeight, Math.ceil(h[i] * (M - y) * 1.0 / w[i])); 
y = 0;
```

```java
import com.sun.rowset.internal.Row;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class Main {
int M, N;
int w[], h[];
Node f[][];

class Node {
    int rowHeight;//行高
    int height;//高度

    Node(int rowHeight, int height) {
        this.rowHeight = rowHeight;
        this.height = height;
    }
}

void solve() {
    int nowHeight = 0;//行高
    int height = 0;//总高
    int y = 0;//当前行位置
    int ans = N * 107;
    for (int i = 0; i < N - 1; i++) {
        if (y == 0) {
            ans = Math.min(ans, height + f[i + 1][0].height + f[i + 1][0].rowHeight);
        } else {
            ans = Math.min(ans, height + f[i + 1][y].height + Math.max(f[i + 1][y].rowHeight, nowHeight));
        }
        if (y + w[i] < M) {
            y += w[i];
            nowHeight = Math.max(nowHeight, h[i]);
        } else if (y + w[i] == M) {
            nowHeight = Math.max(h[i], nowHeight);
            height += nowHeight;
            nowHeight = 0;
            y = 0;
        } else {
            nowHeight = (int) Math.max(nowHeight, Math.ceil(h[i] * (M - y) * 1.0 / w[i]));
            height += nowHeight;
            nowHeight = 0;
            y = 0;
        }
    }
    ans = Math.min(ans, height + nowHeight);
    System.out.println(ans);
}

Main() throws FileNotFoundException {
    Scanner cin = new Scanner(System.in);
    // Scanner cin = new Scanner(new FileInputStream("in.txt"));
    M = cin.nextInt();
    N = cin.nextInt();
    w = new int[N];
    h = new int[N];
    f = new Node[N + 1][M];
    for (int i = 0; i < N; i++) {
        w[i] = cin.nextInt();
        h[i] = cin.nextInt();
    }
    cin.close();
    for (int j = 0; j < M; j++) {
        f[N][j] = new Node(0, 0);
    }
    for (int i = N - 1; i >= 0; i--) {
        for (int j = 0; j < M; j++) {
            if (j + w[i] < M) {
                Node next = f[i + 1][j + w[i]];
                f[i][j] = new Node(Math.max(h[i], next.rowHeight), next.height);
            } else if (j + w[i] == M) {
                Node next = f[i + 1][0];
                f[i][j] = new Node(h[i], next.height + next.rowHeight);
            } else {
                int myh = (int) Math.ceil(h[i] * 1.0 * (M - j) / w[i]);
                Node next = f[i + 1][0];
                f[i][j] = new Node(myh, next.height + next.rowHeight);
            }
        }
    }
    solve();
}

public static void main(String[] args) throws FileNotFoundException {
    new Main();
}
}

```
        