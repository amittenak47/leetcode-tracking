// Last updated: 6/6/2025, 3:32:11 PM
class Solution {
public:
    int popCount(int n) 
    {
        // Count number of bit flips by BK's Algorithm
        // This is known as Brian Kernighan's algorithm for counting set bits. Every iteration removes the lowest set bit from n, so the loop runs exactly popcount(n) times.
        int count;
        for (count = 0; n != 0; ++count) { n &= n - 1; } 
        return count;
    }

    vector<int> countBits(int n) 
    {
        // Get bit count and store in vector index position
        vector<int> ans(n + 1, 0);
        for (int x = 0; x <= n; ++x) { ans[x] = popCount(x); }
        return ans;
    }
};