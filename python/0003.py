class Solution(object):
    def lengthOfLongestSubstring(self, s):
        """
        :type s: str
        :rtype: int
        """
        t = dict()
        ans = 0
        beg = 0
        for i in range(len(s)):
            if s[i] in t and t[s[i]] >= beg:
                beg = t[s[i]] + 1
            t[s[i]] = i
            ans = max(ans, i - beg + 1)
            # print(beg,i,s[i],ans,t)
        return ans


if __name__ == '__main__':
    s = Solution()
    ans = s.lengthOfLongestSubstring("abcabcbb")
    print(ans)

