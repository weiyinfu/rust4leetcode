------------------
title = "hihocoder217周 树形DP"
publishTime = "2018-08-29 11:49:00"
id = "9556492"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho217/problem/1)
一棵树，树中包含TRUE、FALSE、AND、OR四种结点，其中TRUE和FALSE是叶子结点，AND和OR结点的儿子包含多个结点，现在要求执行最少次数的以下操作：
* 把AND变成OR
* 把OR变成AND

使得整棵树的bool值结果翻转。


# 思路
只考虑当前结点和它的儿子。
要想改变当前结点的值，有两种方法：
* 改变当前结点的操作
* 改变当前结点的儿子结点的值

有时甚至需要把这两种方法结合起来，比如：当前结点为AND，它的儿子们全为TRUE。这是要改变当前结点的值，可能要把当前结点改为OR，并且把某个儿子改为FALSE。

```java
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Main {
final int MAX_VLUE = 100000;

class Node {
    List<Node> son = new ArrayList<>();
    String value;
    int parent;
    int reverseValue = -1;
    boolean nowValue = false;
    boolean inited = false;

    Node(String value, int parent) {
        this.value = value;
        this.parent = parent;
    }

    int reverseOne(boolean toValue) {
        int min = MAX_VLUE;
        for (Node i : son) {
            if (i.getValue() == toValue) return 0;
            min = Math.min(min, i.reverse());
        }
        return min;
    }

    int reverseAll(boolean toValue) {
        int s = 0;
        for (Node i : son) {
            if (i.getValue() != toValue) {
                s += i.reverse();
            }
        }
        return s;
    }


    //翻转需要的步数
    int reverse() {
        if (reverseValue != -1) return reverseValue;
        if (value.equals("TRUE") || value.equals("FALSE")) return MAX_VLUE;
        if (value.equals("AND")) {
            if (nowValue) {
                reverseValue = Math.min(reverseOne(false), 1 + reverseAll(false));
            } else {
                reverseValue = Math.min(reverseAll(true), 1 + reverseOne(true));
            }
        } else if (value.equals("OR")) {
            if (nowValue) {
                reverseValue = Math.min(1 + reverseOne(false), reverseAll(false));
            } else {
                reverseValue = Math.min(reverseOne(true), 1 + reverseAll(true));
            }
        }
        return reverseValue;
    }

    boolean getValue() {
        if (inited) return nowValue;
        inited = true;
        if (value.equals("TRUE")) {
            nowValue = true;
            return true;
        } else if (value.equals("FALSE")) {
            nowValue = false;
            return false;
        } else if (value.equals("AND")) {
            nowValue = son.stream().allMatch(Node::getValue);
            return nowValue;
        } else if (value.equals("OR")) {
            nowValue = son.stream().anyMatch(Node::getValue);
            return nowValue;
        }
        return false;
    }
}

Main() {
    Scanner cin = new Scanner(System.in);
    int N = cin.nextInt();
    Node[] a = new Node[N + 1];
    a[0] = new Node("AND", 0);
    for (int i = 1; i <= N; i++) {
        int parent = cin.nextInt();
        String value = cin.next();
        a[i] = new Node(value, parent);
    }
    int root = 1;
    for (int i = 1; i <= N; i++) {
        a[a[i].parent].son.add(a[i]);
        if (a[i].parent == 0) {
            root = i;
        }
    }
    a[root].getValue();
    int step = a[root].reverse();
    if (step >= MAX_VLUE) {
        System.out.println(-1);
    } else {
        System.out.println(step);
    }
}

public static void main(String[] args) {
    new Main();
}
}
```
        