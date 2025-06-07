// Last updated: 6/7/2025, 2:41:51 AM
class Solution {
public:
    int missingNumber(vector<int>& nums) {
        
        // sort(nums.begin(), nums.end()); // (N log N)

        // if (nums[nums.size()-1] != nums.size()) return nums.size(); // Ensure that n is at the last index
        // else if (nums[0] != 0) return 0; // Ensure that 0 is at the first index

        // for (int i = 1; i < nums.size(); i++) {
        //     int expectedNum = nums[i-1] + 1;
        //     if (nums[i] != expectedNum) return expectedNum; // calc expected number at curr index and return if missing
        // }

        // return -1; // Array was not missing any numbers

        // unordered_set<int> numSet;
        // for (int num : nums) numSet.insert(num); // N; add each number to set (1 of each)

        // int expectedNumCount = nums.size() + 1;
        // for (int number = 0; number < expectedNumCount; number++) { // N 
        //     if (numSet.find(number) != numSet.end()) return number; // iterate over set length (expected numbers) searching for expected number
        // }

        // return -1;

        int missing = nums.size(); // XOR every number with its index; if a number is missing, the list size and last number will cancel, leaving the missing number
        for (int i = 0; i < nums.size(); i++) missing ^= i ^ nums[i];
        return missing;
    }
};