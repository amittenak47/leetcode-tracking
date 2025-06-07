// Last updated: 6/7/2025, 2:41:47 AM
class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        // O(N) Time/Space Complexity; Use Set; if a number is duplicated return

        // unordered_set<int> seen;
        // for (auto &num : nums) {
        //     if (seen.count(num)) return num;
        //     else seen.insert(num);
        // }
        // return -1;

        // O(N Log N) / O(N) Time/Space Complexity; Sort the array and check if the prev number is duplicate

        // sort(nums.begin(), nums.end()); 
        // for (int i = 1; i < nums.size(); i++) if (nums[i] == nums[i - 1]) return nums[i];
        // return -1;

        // Binary Search O(N Log N) // Checking small or equal is N 

        // // intuition: bounds are [1, n], any index should have a count that is monotonic (non-decreasing/-increasing) that matches index, 
        // // if count >= expected_index then the smallest number satisfying this property is the dup; mult dups may result in count > expected_index + dup)
        // // everytime count < mid, the left side does not contain numbers rel to the duplicate

        // int low = 1, high = nums.size(), duplicate = -1;
        // while (low <= high) {
        //     int mid = (low + high) / 2;
            
        //     if (small_or_equal(nums, mid) > mid) { 
        //         duplicate = mid;
        //         high = mid - 1;
        //     } else low = mid + 1;
        // }
        // return duplicate;

        // O(N) Iteration; Use array as a Map; range [1, n] maps to indices [1, n] inclusive, and no number will be mapped to index 0

        // the number at index 0 should map to its index (same count as index)
        // while (nums[0] != nums[nums[0]]) swap(nums[0], nums[nums[0]]);
        // return nums[0];

        // O(N) Recursion; Start at 0 and recursively pick up the value and store the current value till the value already exists when storing 
        // return store(nums, 0);

        // int duplicate = 0, n = nums.size() - 1, max_num = *max_element(nums.begin(), nums.end()), max_bit = calcMaxBit(max_num);
        
        // // Iterate over each bit
        // for (int bit = 0; bit < max_bit; bit++) {
            
        //     int mask = (1 << bit), base_count = 0, nums_count = 0;
            
        //     for (int i = 0; i <= n; i++) {
        //         if (i & mask) base_count++; // If bit is set in number (i), add 1 to base_count
        //         if (nums[i] & mask) nums_count++; // If bit is set in nums[i], add 1 to nums_count
        //     }
            
        //     // If the current bit is more frequently set in nums than it is in 
        //     // the range [1, 2, ..., n] then it must also be set in the duplicate number
        //     if (nums_count > base_count) duplicate |= mask;
        // }
        // return duplicate;


        // Find the intersection point of the two runners.
        int dupe1 = nums[0], dupe2 = nums[0];

        do {
            dupe1 = nums[dupe1];
            dupe2 = nums[nums[dupe2]];
        } while (dupe1 != dupe2);

        // Find the "entrance" to the cycle.
        dupe1 = nums[0];
        while (dupe1 != dupe2) {
            dupe1 = nums[dupe1];
            dupe2 = nums[dupe2];
        }

        return dupe1;

    }

    int store(vector<int>& nums, int cur) {
        if (cur == nums[cur]) return cur;
        int nxt = nums[cur];
        nums[cur] = cur;
        return store(nums, nxt);
    }

    int small_or_equal(vector<int> nums, int cur) {
        int count = 0;
        for (auto &num: nums) { if (num <= cur) count++; }
        return count;
    }

    // Find the position of the Most Significant Bit in num
    int calcMaxBit(int num) {
        int bits = 0;
        while (num > 0) { num /= 2; bits++; }
        return bits;
    }
};