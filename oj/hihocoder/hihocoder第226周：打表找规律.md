------------------
title = "hihocoder第226周：打表找规律"
publishTime = "2018-10-27 14:49:00"
id = "9863814"
tags = [ "算法", "hihocoder",]

--------------

[题目列表](http://hihocoder.com/contest/hiho226/problems)
# 问题描述
有一个文本框，可以执行以下操作：
* 输入A
* Ctrl+C 复制
* Ctrl+V 粘贴
* Ctrl+A 全选

N次操作最多能够造出多少个A来？
输入一个N，输出一个整数，表示最多有多少个A。因为输出结果太大，需要模上1e9+7。

# 暴力方法
定义一个Node，use表示操作次数，count表示当前的A的个数，paste表示粘贴板上的A的个数。
```java
class Node {
    int use;
    int count;
    int paste;

    Node(int use, int count, int paste) {
        this.use = use;
        this.count = count;
        this.paste = paste;
    }
}
```
使用优先队列，按照use从小到大进行扩展。对于每个结点，根据use和count更新状态。
复杂度极高，只能求到20左右。

# 动态规划方法
思考四种操作之间的关系，可以发现以下贪心法则：
* ctrl+A，ctrl+C，ctrl+V必然是一气呵成的
* 只要剪贴板上有东西，就不可能直接输入A。直接输入A只能获取一个字符，ctrl+V则可以获取至少一个。

定义f[n]表示n个操作的最大长度，只需要假设最后一次操作为i（i<n-2），那么`f[n]=f[i]*(n-i-1)`，表示执行n-i-1次粘贴操作。使用last[n]表示n个操作的上次复制时机。打表很容易发现规律。

对于`n<16`，没有明确规律。
对于`n>=16`，`f[n]=f[n-5]*4`

找到了规律，就很容易通过计算的方式求解了。

# 数学方法
```java
import java.util.Scanner;

public class Main {
long mod = 1000000007;
long[] a = new long[100];
int[] last = new int[a.length];

long pow(long x, int y) {
    if (y == 0) return 1;
    if (y == 1) return x;
    long z = pow(x, y / 2);
    if (y % 2 == 0) {
        return z * z % mod;
    } else {
        return z * z * x % mod;
    }
}

long solve(int x) {
    if (x <= 16) return a[x];
    int power = (x - 11) / 5;
    int which = (x - 11) % 5;
    long ans = a[11 + which] * pow(4, power) % mod;
    return ans;
}

Main() {
    Scanner cin = new Scanner(System.in);
    int x = cin.nextInt();

    for (int i = 0; i < 6; i++) {
        a[i] = i;
    }
    for (int i = 6; i <= 16; i++) {
        for (int j = 1; j < i - 2; j++) {
            long now = a[j] * (i - 1 - j);
            if (now > a[i]) {
                a[i] = now;
                last[i] = j;
            }
        }
    }
    System.out.println(solve(x));
}

public static void main(String[] args) {
    new Main();
}
}
```
# 总结
一直以来，数学中最奇妙的东西都是像这道题所体现出来的那样。
在数据较小的时候毫无规律，当数据达到一定程度之后，规律突然“冒”出来了。
这是最神奇的事物。这是这道题第一个有趣的地方。
第二个有趣的地方是，简单的定义引出复杂的结论。
这道题非常切合实际，大部分程序员在某个瞬间应该都闪念过这个问题，但是没有深究。谁能想到简单的题设背后蕴含着如此复杂精致的规律。
        