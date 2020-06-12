class Solution:
    def simplifyPath(self, path):
        """
        :type path: str
        :rtype: str
        """
        a = path.split("/")
        now = []
        for i in a:
            if not i: continue
            if i == '.':
                continue
            elif i == '..':
                if now:
                    now.pop()
            else:
                now.append(i)
        return '/' + '/'.join(now)


if __name__ == '__main__':
    print(Solution().simplifyPath("/a/../b"))
