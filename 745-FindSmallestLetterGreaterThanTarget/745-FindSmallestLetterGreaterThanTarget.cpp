// Last updated: 6/6/2025, 3:31:50 PM
class Solution {
public:
    char nextGreatestLetter(vector<char>& letters, char target) {
        
        int left = 0, right = letters.size() - 1, mid;
        
        while (left <= right) {
            
            mid = left + (right - left) / 2;

            if (letters[mid] <= target) left = mid + 1; // if target == mid (seeking next greatest letter), shift mid +1
            else right = mid - 1; // if target > mid (and != mid), shift mid -1
        
        }
        return left == letters.size() ? letters[0] : letters[left]; // letters[0] smallest letter if target < all letters, or smallest letter available

        // // Brute Force (N)
        // for (auto letter : letters) if (letter > target) return letter;
        // return letters[0];
    }
};