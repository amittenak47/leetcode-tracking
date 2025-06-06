// Last updated: 6/6/2025, 3:31:37 PM
class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        // int n = nums.size(), l = 0, r = n - 1;
        // vector<int> result(n);

        // for (int i = n - 1; i >= 0; i--) {
        //     if (abs(nums[l]) < abs(nums[r]) ) {
        //         result[i] = nums[r] * nums[r];
        //         --r;
        //     } else {
        //         result[i] = nums[l] * nums[l];
        //         ++l;
        //     }
        // }

        // return result;

        size_t n = nums.size();
        vector<int> ans(n);
        for (size_t i = 0; i < n; i++) {
            ans[i] = nums[i] * nums[i];
        }

        sort(ans.begin(), ans.end());
        return ans;
    }
};