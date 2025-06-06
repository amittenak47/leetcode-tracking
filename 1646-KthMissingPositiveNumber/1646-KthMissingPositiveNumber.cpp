// Last updated: 6/6/2025, 3:31:24 PM
class Solution {
public:
    int findKthPositive(vector<int>& arr, int k) {

        int left = 0, right = arr.size() - 1;
        while (left <= right) {
            int pivot = left + (right - left) / 2;
            if (arr[pivot] - pivot - 1 < k) { left = pivot + 1; }
            else { right = pivot - 1; }
        }
        return left + k;
    }
};