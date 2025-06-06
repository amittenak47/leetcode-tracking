// Last updated: 6/6/2025, 3:31:58 PM
class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {

        unordered_map<int, bool> hashTable;
        
        for (int i = 0; i < nums.size(); i++) {
            hashTable.insert(make_pair(nums[i], true));
            int newIndex = abs(nums[i]) - 1;
            if (nums[newIndex] > 0) { nums[newIndex] *= -1; }
        }
        
        vector<int> result;
        for (int i = 1; i <= nums.size(); i++) {
            if (nums[i - 1] > 0) { result.push_back(i); }
        }
        
        return result;
    }
};