class Solution:
    def mergeAlternately(self, word1: str, word2: str) -> str:
        w1 = len(word1)
        w2 = len(word2)
        maxlen = max(w1, w2)
        minlen = min(w1, w2)

        res = ""
        for x in range(maxlen):
            if x < w1:
                res += word1[x]
            if x < w2:
                res += word2[x]

        for x in range(minlen):
            res += word1[x]
            res += word2[x]

        return res + (word1[w2:] if w1 > w2 else word2[w1:])
