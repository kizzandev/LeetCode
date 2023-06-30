// Added
#include <string>
#include <iostream>
using namespace std;

// Solution
class Solution
{
public:
    string gcdOfStrings(string &str1, string &str2)
    {
        // First let's make sure the str2 is the shortest string
        if (str1.length() < str2.length())
            swap(str1, str2);
        if (str1.find(str2) == string::npos || str1 + str2 != str2 + str1) // There's no occurrance or not div-able
            return "";

        // If I find the str2, reduce str1
        while (str1.find(str2) == 0 && !str2.empty())
        {
            str1 = str1.substr(str2.length());
            if (str1.length() < str2.length())
                swap(str1, str2);
        }

        return str1;
    }
};

// Test
int main()
{
    // string str1 = "ABCABC";
    // string str2 = "ABC";
    // string str1 = "ABABAB";
    // string str2 = "ABAB";
    // string str1 = "LEET";
    // string str2 = "CODE";
    string str1 = "ABCDEF";
    string str2 = "ABC";
    cout << Solution().gcdOfStrings(str1, str2) << endl; // "ABC"
    return 0;
}