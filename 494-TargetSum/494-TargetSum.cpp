// Last updated: 6/6/2025, 3:31:57 PM
class Solution {
public:
    int total;
    
    int findTargetSumWays(vector<int>& nums, int target) {
        
        int total = accumulate(nums.begin(), nums.end(), 0);
        vector<int> dp(2 * total + 1);
        dp[nums[0] + total] = 1;
        dp[-nums[0] + total] += 1;
        
        for (int i = 1; i < nums.size(); i++) {
            vector<int> next(2 * total + 1);
            for (int sum = -total; sum <= total; sum++) {
                if (dp[sum + total] > 0) {
                    next[sum + nums[i] + total] += dp[sum + total];
                    next[sum - nums[i] + total] += dp[sum + total];
                }
            }
            dp = next;
        }
        
        return abs(target) > total ? 0 : dp[target + total];
    }
};