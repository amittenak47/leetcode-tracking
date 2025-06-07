// Last updated: 6/7/2025, 2:41:56 AM
class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {

        int length = nums.size();
        // vector<int> L(length, 0), R(length, 0), answer(length, 0);

        // L[0] = 1; // L[i] contains the product of all left elements
        // for (int i = 1; i < length; i++) { 
        //     // L[i - 1] already contains the product of elements to the left of 'i - 1'
        //     // Multiply it with nums[i - 1] to get the product of all elements left of index 'i'
        //     L[i] = nums[i - 1] * L[i - 1];
        // }
        // R[length - 1] = 1; // R[i] contains the product of all right elements
        // for (int i = length - 2; i >= 0; i--) {
        //     // R[i - 1] already contains the product of elements to the right of 'i - 1'
        //     // Multiply it with nums[i - 1] to get the product of all elements right of index 'i'
        //     R[i] = nums[i + 1] * R[i + 1];
        // }

        // for (int i = 0; i < length; i++) {
        //     answer[i] = L[i] * R[i]; // L[i] and R[i] edges are both product without self, else multiple left and right
        // }

        // return answer;

        // O(1) Space Complexity: Use the output array to build one direction, and a variable to build the other 
        vector<int> answer(length, 0);

        answer[0] = 1;
        for (int i = 1; i < length; i++) answer[i] = nums[i - 1] * answer[i - 1];

        int R = 1; // use a separate var to store the product of right elements
        for (int i = length - 1; i >= 0; i--) {
            answer[i] = answer[i] * R;
            R *= nums[i];
        }

        return answer;
    }
};