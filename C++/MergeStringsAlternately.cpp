// Included to avoid errors
#include <string>
using namespace std;

// Solution
class Solution
{
public:
    string mergeAlternately(string &word1, string &word2)
    {
        const short w1 = word1.length();        // length of word1
        const short w2 = word2.length();        // length of word1
        const short minLen = w1 < w2 ? w1 : w2; // min length

        string res;

        for (short i = 0; i < minLen; i++)
            res = res + word1[i] + word2[i];

        if (w1 > w2)
            res += word1.substr(minLen);
        else
            res += word2.substr(minLen, w2 - 1);

        return res;
    }
};
