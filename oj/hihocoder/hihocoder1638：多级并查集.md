------------------
title = "hihocoder 1638：多级并查集"
publishTime = "2018-05-27 17:36:00"
id = "9098299"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](https://hihocoder.com/problemset/problem/1638)
并查集可以用于聚类。

```java
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.util.Scanner;

class Main {
int N = (int) (1e4 + 7);
int father[] = new int[N << 1];

class FatherDis {
    int father;
    int dis;

    FatherDis(int father, int dis) {
        this.father = father;
        this.dis = dis;
    }

    @Override
    public String toString() {
        return String.format("(father=%d,dis=%d)", father, dis);
    }
}

FatherDis find(int x) {
    if (father[x] == x) {
        return new FatherDis(x, 0);
    }
    FatherDis f = find(father[x]);
    f.dis++;
    if ((f.dis & 1) == 0) {
        father[x] = f.father + N;
    } else {
        father[x] = f.father;
    }
    return f;
}

Main() {
    Scanner cin = new Scanner(System.in); 
    int t = cin.nextInt();
    while (t-- > 0) {
        int n = cin.nextInt(), m = cin.nextInt();
        for (int i = 1; i <= n; i++) father[i] = father[i + N] = i;
        int fail = -1;
        for (int i = 0; i < m; i++) {
            int x = cin.nextInt(), u = cin.nextInt(), v = cin.nextInt();
            if (fail != -1) continue;//读完数据再说
            FatherDis fu = find(u), fv = find(v);
            boolean sameLevel = (fu.dis & 1) == (fv.dis & 1);
            if (x == 0) {
                if (fu.father != fv.father) {
                    if (sameLevel) {
                        father[fu.father] = fv.father + N;
                    } else {
                        father[fu.father] = fv.father;
                    }
                } else {
                    if (!sameLevel) {//不同类别
                        fail = i;
                    }
                }
            } else {
                if (fu.father != fv.father) {
                    if (sameLevel) {
                        father[fu.father] = fv.father;
                    } else {
                        father[fu.father] = fv.father + N;
                    }
                } else {
                    if (sameLevel) {//同一类别
                        fail = i;
                    }
                }
            }
        }
        if (fail == -1) {
            System.out.println("great");
        } else {
            System.out.println("sad");
            System.out.println(fail + 1);
        }
    }
}

public static void main(String[] args) {
    new Main();
}
}
```
        