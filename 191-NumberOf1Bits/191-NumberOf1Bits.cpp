// Last updated: 6/7/2025, 2:42:12 AM
class Solution {
public:
     int hammingWeight(uint32_t n) {
        int count = 0;
         
        for (int i = 0; i < 32; i++) {
            count = (n >> i) & 1 ? count + 1 : count;
        }

        return count;     
    }
};