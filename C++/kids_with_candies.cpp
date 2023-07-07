// Added
// #include <string>   // No strings as far as the eye can C
// #include <iostream> // If you want to test it
#include <vector>
#include <algorithm> // max_element()
using namespace std;

// Solution
class Solution
{
public:
    vector<bool> kidsWithCandies(vector<int> &candies, int &extraCandies)
    {
        // Get the max element of a vector
        int greatest = *max_element(begin(candies), end(candies));
        vector<bool> res;
        // Iter once and push true|false accordingly
        for (int &c : candies)
            res.push_back((c + extraCandies) >= greatest);
        return res;
    }
};
