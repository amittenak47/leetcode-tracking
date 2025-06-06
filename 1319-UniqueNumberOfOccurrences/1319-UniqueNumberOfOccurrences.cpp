// Last updated: 6/6/2025, 3:31:30 PM
class Solution {
public:
    bool uniqueOccurrences(vector<int> &arr) {
        unordered_map<int, int> hashmap;
        unordered_set<int> hashset;
        for (int n : arr) ++hashmap[n];
        for (pair<int, int> p : hashmap)
            if (!hashset.insert(p.second).second) return false;
        return true;
    }
};