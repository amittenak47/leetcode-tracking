// Last updated: 6/7/2025, 2:42:01 AM
class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        std::unordered_set<int> set;
        for (auto &x: nums) {
            if (set.find(x) != set.end()) { return true; }
            set.insert(x);
        }
        return false;
    }
};