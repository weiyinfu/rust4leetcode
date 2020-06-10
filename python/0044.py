class Solution:
    def isMatch(self, s, p):
        """
        :type s: str
        :type p: str
        :rtype: bool
        """
        a = []
        now = ''
        p = '^' + p + "$"
        s = '^' + s + '$'
        for i in p:
            if i == '*':
                if len(now):
                    a.append(now)
                    now = ''
            else:
                now = now + i
        if len(now):
            a.append(now)
        i = 0
        j = 0
        while i < len(s) and j < len(a):
            if self.ok(s, i, a[j]):
                i += len(a[j])
                j += 1
            else:
                i += 1
        return j == len(a) and i == len(s)

    def ok(self, s, i, ss):
        if i + len(ss) > len(s): return False
        for j in range(len(ss)):
            if ss[j] != s[i + j] and not ss[j] == '?':
                return False
        return True


if __name__ == '__main__':
    s = Solution()
    for i in (("a", "*"),
              ("acb", '*a?b*'),
              (
                      "babbbbaabababaabbababaababaabbaabababbaaababbababaaaaaabbabaaaabababbabbababbbaaaababbbabbbbbbbbbbaabbb",
                      "b**bb**a**bba*b**a*bbb**aba***babbb*aa****aabb*bbb***a")):
        print(i[0], i[1], s.isMatch(i[0], i[1]))
