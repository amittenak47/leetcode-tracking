// Last updated: 6/6/2025, 3:31:39 PM
class Solution {
public:
    int totalFruit(vector<int>& fruits) {

        // unordered_map<int, int> basket;
        // int left, right;
        
        // for (left = 0, right = 0; right < fruits.size(); ++right) {
        //     basket[fruits[right]]++; // increment right pointer
            
        //     if (basket.size() > 2) { // decrement left pointer if exceeding size limit 
        //         basket[fruits[left]]--;
        //         if (basket[fruits[left]] == 0) basket.erase(fruits[left]); // erase if none in current window are stored in map
        //         left++;
        //     }
        // }

        // return right - left; // return difference in pointer index as length

        unordered_map<int, int> basket;
        int  maxPicked = 0;
        for (int left = 0, right = 0; right < fruits.size(); ++right) {
            basket[fruits[right]]++; // increment right pointer
            
            while (basket.size() > 2) { // decrement left pointer if exceeding size limit 
                basket[fruits[left]]--;
                if (basket[fruits[left]] == 0) basket.erase(fruits[left]); // erase if none in current window are stored in map
                left++;
            }

            maxPicked = max(maxPicked, right - left + 1);
        }

        return maxPicked;
    }
};