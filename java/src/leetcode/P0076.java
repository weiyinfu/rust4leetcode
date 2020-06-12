import java.util.*;

class Solution {

class Point {
    int x, y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    int size() {
        return this.y - this.x;
    }
}

void printMap(Map<Character, Integer> ma) {
    for (char c : ma.keySet()) {
        System.out.println(c + ":" + ma.get(c));
    }
    System.out.println("======");
}

Point go(char[] s, Map<Character, Integer> t) {
    HashMap<Character, Integer> charMap = new HashMap<>();
    for (char i : t.keySet()) {
        charMap.put(i, 0);
    }
    int r = 0;
    int visited = 0;
    while (r < s.length) {
        if (t.containsKey(s[r])) {
            charMap.put(s[r], charMap.get(s[r]) + 1);
            if (charMap.get(s[r]).equals(t.get(s[r]))) {
                visited++;
                if (visited == charMap.size()) {
                    break;
                }
            }
        }
        r++;
    }
    //use close region
    Point ans = new Point(0, r);
    for (int l = 0; l < s.length; l++) {
        if (!t.containsKey(s[l])) continue;
//        System.out.println("left=" + l + " right=" + r + "  " + new String(Arrays.copyOfRange(s, l, r + 1)));
//        printMap(charMap);
        Point now = new Point(l, r);
        if (now.size() < ans.size()) {
            ans = now;
        }
        charMap.put(s[l], charMap.get(s[l]) - 1);
        if (charMap.get(s[l]) < t.get(s[l])) {
            //寻找下一个s[l]
            r++;
            while (r < s.length) {
                if (t.containsKey(s[r])) {
                    charMap.put(s[r], charMap.get(s[r]) + 1);
                }
                if (s[r] == s[l]) break;
                r++;
            }
            if (r >= s.length) break;//无法弥补s[l]的离去
        }
    }
    return ans;
}

Map<Character, Integer> tomap(String s) {
    HashMap<Character, Integer> charset = new HashMap<>();
    for (char c : s.toCharArray()) {
        charset.put(c, charset.getOrDefault(c, 0) + 1);
    }
    return charset;
}


public String minWindow(String s, String t) {
    Map<Character, Integer> sset = tomap(s);
    Map<Character, Integer> tset = tomap(t);
    for (char c : tset.keySet()) {
        if (tset.get(c) > sset.getOrDefault(c, 0)) {
            return "";
        }
    }
    Point ans = go(s.toCharArray(), tset);
    return s.substring(ans.x, ans.y + 1);
}

public static void main(String[] args) {
    String s = "bba";
    String t = "ab";
    Solution so = new Solution();
    String ans = so.minWindow(s, t);
    System.out.println(ans);
}
}