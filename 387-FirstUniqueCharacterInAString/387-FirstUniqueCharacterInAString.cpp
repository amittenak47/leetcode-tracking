// Last updated: 6/6/2025, 3:32:03 PM
class Solution {
public:
    int firstUniqChar(string s) {
        
        // Hashmap O(N)
        int n = s.length();
        unordered_map<char, int> count; // build hash map : character and how often it appears
        for (int i = 0; i < n; i++) {
            char c = s.at(i);
            count[c] = (count.emplace(c, 0).first)->second + 1;
        }
        
        for (int i = 0; i < n; i++) { // find the index
            if (count[s.at(i)] == 1) return i;
        }

        return -1;
    }
};