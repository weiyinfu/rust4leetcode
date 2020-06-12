class Solution:
    def write(self, s, word, pos):
        for i in range(len(word)):
            s[pos + i] = word[i]

    def justify(self, words, maxWidth):
        sep = maxWidth - sum(len(i) for i in words)
        s = [' '] * maxWidth
        self.write(s, words[0], 0)
        sepCount = len(words) - 1
        if sepCount == 0: return ''.join(s)
        freeCount = sep % sepCount
        i = len(words[0]) + sep // sepCount + (1 if freeCount else 0)
        if freeCount: freeCount -= 1
        for j in range(1, len(words)):
            self.write(s, words[j], i)
            i += len(words[j]) + sep // sepCount
            if freeCount:
                i += 1
                freeCount -= 1
        return ''.join(s)

    def fullJustify(self, words, maxWidth):
        """
        :type words: List[str]
        :type maxWidth: int
        :rtype: List[str]
        """
        ans = []
        i = 0
        l = 0
        wordCount = 0
        while i < len(words):
            l += len(words[i])
            wordCount += 1
            if l + wordCount - 1 > maxWidth:
                ans.append(self.justify(words[i + 1 - wordCount:i], maxWidth))
                l = 0
                wordCount = 0
            else:
                i += 1

        s = [' '] * maxWidth
        j = 0
        for i in range(len(words) - wordCount, len(words)):
            self.write(s, words[i], j)
            j += len(words[i])
            if i < len(words) - 1:
                j += 1
        ans.append(''.join(s))
        return ans


if __name__ == '__main__':
    print(Solution().fullJustify(["a", "b", "c", "d", "e"], 3))
"""
["Don't  go  around  saying  the","world   owes you a living; the","world owes you nothing; it was","here first.                   "]
["Don't  go  around  saying  the","world  owes  you a living; the","world owes you nothing; it was","here first.                   "]
"""
