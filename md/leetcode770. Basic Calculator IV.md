------------------
title = "leetcode770. Basic Calculator IV"
publishTime = "2018-03-04 10:57:00"
id = "8505541"
tags = [ "算法", "leetcode",]

--------------

此题真可谓是练习编程语言的绝好材料 ！

```java

import java.util.*;

class Solution {
class Item {
    Map<String, Integer> vars = new TreeMap<>();
    int k = 1;

    String tos() {
        StringBuilder builder = new StringBuilder();
        List<String> a = new ArrayList<>();
        for (String i : vars.keySet()) {
            for (int j = 0; j < vars.get(i); j++) {
                a.add(i);
            }
        }
        if (a.size() == 0) return "";
        builder.append(a.get(0));
        for (int i = 1; i < a.size(); i++) {
            builder.append("*" + a.get(i));
        }
        return builder.toString();
    }

    Item copy() {
        Item it = new Item();
        it.k = k;
        for (Map.Entry<String, Integer> i : vars.entrySet()) {
            it.vars.put(i.getKey(), i.getValue());
        }
        return it;
    }

    Item mul(Item another) {
        Item ans = new Item();
        ans.k = k * another.k;
        for (Map.Entry<String, Integer> i : vars.entrySet()) {
            ans.vars.put(i.getKey(), i.getValue());
        }
        for (Map.Entry<String, Integer> i : another.vars.entrySet()) {
            if (ans.vars.containsKey(i.getKey())) {
                ans.vars.put(i.getKey(), ans.vars.get(i.getKey()) + i.getValue());
            } else {
                ans.vars.put(i.getKey(), i.getValue());
            }
        }
        return ans;
    }

    int getMi() {
        int s = 0;
        for (int x : vars.values()) {
            s += x;
        }
        return s;
    }

    Item render(Map<String, Integer> eval) {
        Item j = new Item();
        j.k = k;
        for (Map.Entry<String, Integer> i : vars.entrySet()) {
            if (eval.containsKey(i.getKey())) {
                j.k *= Math.pow(eval.get(i.getKey()), i.getValue());
            } else {
                j.vars.put(i.getKey(), i.getValue());
            }
        }
        return j;
    }
}

class Exp {
    List<Item> items = new ArrayList<>();

    Exp add(Exp another) {
        Exp exp = new Exp();
        Map<String, Item> ma = new TreeMap<>();
        for (Item i : another.items) {
            Item j = i.copy();
            exp.items.add(j);
            ma.put(j.tos(), j);
        }
        for (Item i : items) {
            String k = i.tos();
            if (ma.containsKey(k)) {
                ma.get(k).k += i.k;
            } else {
                Item j = i.copy();
                ma.put(k, j);
                exp.items.add(j);
            }
        }
        return exp;
    }

    Exp sub(Exp another) {
        another.mul(-1);
        Exp ans = add(another);
        another.mul(-1);
        ans.mul(-1);
        return ans;
    }

    void mul(int x) {
        for (Item i : items) {
            i.k *= x;
        }
    }

    Exp mul(Exp another) {
        Exp ans = new Exp();
        for (Item i : items) {
            for (Item j : another.items) {
                ans.items.add(i.mul(j));
            }
        }
        return ans;
    }

    @Override
    public String toString() {
        List<String> a = tos(this);
        StringBuilder builder = new StringBuilder();
        for (String i : a) {
            builder.append("+" + i);
        }
        return builder.toString();
    }
}

List<String> tokenize(String expression) {
    List<String> ans = new ArrayList<>();
    for (int i = 0; i < expression.length(); i++) {
        if ("()+-*".indexOf(expression.charAt(i)) != -1) {
            ans.add(expression.charAt(i) + "");
        } else if (Character.isDigit(expression.charAt(i))) {
            int j = i;
            while (j < expression.length() && Character.isDigit(expression.charAt(j))) {
                j++;
            }
            ans.add(expression.substring(i, j));
            i = j - 1;
        } else if (Character.isLetter(expression.charAt(i))) {
            int j = i;
            while (j < expression.length() && Character.isLetter(expression.charAt(j))) {
                j++;
            }
            ans.add(expression.substring(i, j));
            i = j - 1;
        }
    }
    return ans;
}

int getPriority(String i) {
    if (i.equals("*")) return 1;
    if (i.equals("+") || i.equals("-")) return 0;
    if (i.equals("(")) return 2;
    if (i.equals(")")) return -1;
    throw new RuntimeException("无法操作符号" + i);
}

boolean isNumber(String s) {
    for (int i = 0; i < s.length(); i++) {
        if (!Character.isDigit(s.charAt(i))) {
            return false;
        }
    }
    return true;
}

Exp number2Exp(int x) {
    Exp exp = new Exp();
    Item it = new Item();
    it.k = x;
    exp.items.add(it);
    return exp;
}

Exp var2Exp(String var) {
    Exp exp = new Exp();
    Item it = new Item();
    it.vars.put(var, 1);
    exp.items.add(it);
    return exp;
}

Exp calculate(Exp m, Exp n, String op) {
    if (op.equals("*")) return m.mul(n);
    else if (op.equals("+")) return m.add(n);
    else if (op.equals("-")) return m.sub(n);
    throw new RuntimeException("无法处理" + op + "运算");
}

Exp parse(List<String> token) {
    Stack<Exp> num = new Stack<>();
    Stack<String> op = new Stack<>();
    for (String i : token) {
        if (i.equals(")")) {
            while (!op.peek().equals("(")) {
                num.push(calculate(num.pop(), num.pop(), op.pop()));
            }
            op.pop();
        } else if ("+-*(".indexOf(i.charAt(0)) != -1) {
            while (!op.empty() && !op.peek().equals("(") && getPriority(i) <= getPriority(op.peek())) {
                Exp a = num.pop(), b = num.pop();
                String o = op.pop();
                num.push(calculate(a, b, o));
            }
            op.push(i);
        } else if (isNumber(i)) {
            num.push(number2Exp(Integer.parseInt(i)));
        } else {
            num.push(var2Exp(i));
        }
    }
    while (!op.empty()) {
        num.push(calculate(num.pop(), num.pop(), op.pop()));
    }
    return num.pop();
}

Exp render(Exp exp, Map<String, Integer> eval) {
    Exp ans = new Exp();
    for (Item i : exp.items) {
        Item j = i.render(eval);
        ans.items.add(j);
    }
    return ans;
}

Exp sort(Exp exp) {
    //合并同类项
    Map<String, Item> vars = new TreeMap<>();
    for (Item i : exp.items) {
        if (vars.containsKey(i.tos())) {
            vars.get(i.tos()).k += i.k;
        } else {
            vars.put(i.tos(), i);
        }
    }
    //去掉0项
    exp.items = new ArrayList<>();
    for (Item i : vars.values()) {
        if (i.k == 0) continue;
        exp.items.add(i);
    }
    //排序
    exp.items.sort((o1, o2) -> {
        int mi1 = o1.getMi();
        int mi2 = o2.getMi();
        if (mi1 == mi2) {
            return o1.tos().compareTo(o2.tos());
        } else {
            return mi2 - mi1;
        }
    });
    return exp;
}

List<String> tos(Exp exp) {
    List<String> ans = new ArrayList<>();
    for (Item i : exp.items) {
        String it = i.k + "";
        if (i.tos().length() > 0) {
            it += "*" + i.tos();
        }
        ans.add(it);
    }
    return ans;
}


public List<String> basicCalculatorIV(String expression, String[] evalvars, int[] evalints) {
    List<String> token = tokenize(expression);
    Map<String, Integer> eval = new TreeMap<>();
    for (int i = 0; i < evalvars.length; i++) {
        eval.put(evalvars[i], evalints[i]);
    }
    Exp exp = parse(token);
    exp = render(exp, eval);
    exp = sort(exp);
    List<String> ans = tos(exp);
    return ans;
}

void haha() {
    List<String> ans = basicCalculatorIV("a*b+a*b*c*d", new String[]{"b"}, new int[]{2});
    for (String i : ans) {
        System.out.print(i + " ");
    }
}

public static void main(String[] args) {
    Solution s = new Solution();
    s.haha();
}
}
```
        