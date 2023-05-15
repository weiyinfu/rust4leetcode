------------------
title = "hihocoder第218周：AC自动机"
publishTime = "2018-09-07 09:03:00"
id = "9605860"
tags = [ "算法", "hihocoder",]

--------------

[题目链接](http://hihocoder.com/contest/hiho218/problems)

# 问题描述
给定n个单词，给定一个长字符串s，单词总长度和字符串s的长度都不超过1e5。要求把s中所有的出现单词的位置用`*`替代。
例如：
样例输入
```
2  
abc  
cd  
abcxyzabcd
```
样例输出
```
***xyz****
```

关键一点在于：先找到应该打`*`的全部字符，然后再统一改写成`*`，也就是要考虑abcd同时命中abc和cd的情况。

# 思路
AC自动机是算法世界中最美妙的事物之一。它像一个大合唱一样，过去的KMP、字典树、树形DP、有限状态自动机一股脑地来了，聚合在一起，最终完美地达到了O（N）的时间复杂度。

像KMP一样，关键在于求fail指针。AC自动机相比字典树什么也没多，仅仅多了一堆fail指针，让结点和结点之间的联系变得紧密，神奇恰恰发生在一对乱指的指针上。

算法就是玩指针，有时候一根指针，有时候多根指针；有时候从前往后走，有时候从后往前走，有时候转圈走；有时候一步一步走，有时候一片一片走。

求fail指针的过程跟KMP算法非常类似，只不过一变多。最关键的是一开始的时候要假设已经fail了，把root结点的儿子们的fail初始化为root，然后就可以往后走了。

此题一个比较隐蔽的case：
```
2
abcde
ab
abcdxabcdm
```
应该输出`**cdx**cdm`，注意初始化fail的时候要把该结点是否为terminal考虑进去，并对结点是否为terminal进行改写。


# 代码
```java

import java.util.*;

public class Main {
//字典树结点
class TrieNode {
    char ch;//此值仅用于调试
    TrieNode[] sons;
    TrieNode fail;
    char[] value;//如果是terminal，则nodeValue不为空

    boolean isTerminal() {
        return value != null;
    }

    TrieNode(char ch) {
        this.ch = ch;
    }
}

//命中：pos表示命中的最后位置，s表示命中的单词
class Hit {
    int beg;
    char[] s;

    Hit(char[] s, int beg) {
        this.s = s;
        this.beg = beg;
    }
}


class Trie {
    TrieNode root = new TrieNode(' ');

    Trie(char[][] patterns) {
        for (char[] i : patterns) insert(i);
        build();
    }

    void insert(char[] s) {
        TrieNode now = root;
        for (char c : s) {
            //遇山开路，遇水铺桥
            if (now.sons == null) {
                now.sons = new TrieNode[26];
            }
            if (now.sons[c - 'a'] == null) {
                now.sons[c - 'a'] = new TrieNode(c);
            }
            now = now.sons[c - 'a'];
        }
        now.value = s;
    }

    List<Hit> query(char[] s) {
        List<Hit> hits = new ArrayList<>(maxn);
        TrieNode now = null;
        for (int i = 0; i < s.length; i++) {
            char c = s[i];
            if (now == null) now = root;
            while (now != root) {
                if (now.sons == null || now.sons[c - 'a'] == null) {
                    now = now.fail;
                    continue;
                }
                break;
            }
            now = now.sons[c - 'a'];
            if (now == null) {
                now = root;
                continue;
            }
            if (now.isTerminal()) hits.add(new Hit(now.value, i - now.value.length + 1));
        }
        return hits;
    }

    TrieNode getFail(TrieNode pre, int ch) {
        while (pre != root) {
            if (pre.sons != null && pre.sons[ch] != null)
                break;
            pre = pre.fail;
        }
        if (pre.sons[ch] != null) return pre.sons[ch];
        return root;
    }

    void build() {
        Queue<TrieNode> q = new LinkedList<>();
        root.fail = root;
        //初始化第一层，假设一开始没命中，之后应该怎么办
        /**
         * 某种程度上，AC自动机相当于动态规划
         * */
        for (TrieNode i : root.sons) {
            if (i != null) {
                q.add(i);
                i.fail = root;
            }
        }
        while (!q.isEmpty()) {
            TrieNode now = q.poll();
            if (now.sons == null) continue;
            for (int i = 0; i < now.sons.length; i++) {
                if (now.sons[i] == null) continue;
                now.sons[i].fail = getFail(now.fail, i);
                //如果我不是终点，我需要把自己设置成终点
                /**
                 * 此处非常关键
                 * */
                if (now.sons[i].fail.isTerminal() && !now.sons[i].isTerminal()) {
                    now.sons[i].value = now.sons[i].fail.value;
                }
                q.add(now.sons[i]);
            }
        }

//        show(root);
    }
}

class Node {
    int x, type;

    Node(int x, int type) {
        this.x = x;
        this.type = type;
    }
}

final int maxn = 1007;
int[] lens;

Main() {
    Scanner cin = new Scanner(System.in);
    int n = cin.nextInt();
    lens = new int[n];
    char[][] patterns = new char[n][];
    for (int i = 0; i < n; i++) {
        patterns[i] = cin.next().toCharArray();
    }
    Trie tree = new Trie(patterns);
    char[] s = cin.next().toCharArray();
    List<Hit> hits = tree.query(s);
    List<Node> nodes = new ArrayList<>(2 * hits.size());
    for (Hit i : hits) {
        nodes.add(new Node(i.beg, 1));
        nodes.add(new Node(i.beg + i.s.length, -1));
    }
    nodes.sort(Comparator.comparing(x -> x.x));
    StringBuilder builder = new StringBuilder();
    int in = 0;
    int j = 0;
    for (int i = 0; i < s.length; i++) {
        while (j < nodes.size() && nodes.get(j).x <= i) {
            in += nodes.get(j).type;
            j++;
        }
        if (in == 0) builder.append(s[i]);
        else builder.append('*');
    }
    System.out.println(builder.toString());
}

public static void main(String[] args) {
    new Main();
}
}
```
        