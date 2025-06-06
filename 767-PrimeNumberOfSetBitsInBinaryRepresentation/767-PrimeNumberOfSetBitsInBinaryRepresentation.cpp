// Last updated: 6/6/2025, 3:31:47 PM
class Solution {
public:
    int countPrimeSetBits(int left, int right) {
        // 2, 3, 5, 7, 11, 13, 17, 19
        int prime = stoi("10100010100010101100", 0, 2);
        int count = 0;
        
        while (left <= right) { count += prime >> bitset<32>(left++).count() & 1; }
        return count;

    }
};