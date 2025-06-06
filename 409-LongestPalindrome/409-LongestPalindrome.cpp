// Last updated: 6/6/2025, 3:32:00 PM
class Solution {
public:
    int longestPalindrome(string s) {
        int count[128] = {0};
        char* char_arr = s.data();
        for (int i = 0; i < s.length(); i++) {
            char c = char_arr[i];
            count[int(c)]++;
        }

        int ans = 0;
        for (int v: count) {
            ans += v / 2 * 2;
            if (ans % 2 == 0 && v % 2 == 1)
                ans++;
        }
        return ans;        
    }
};